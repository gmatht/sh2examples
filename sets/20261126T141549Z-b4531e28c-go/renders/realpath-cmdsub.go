package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "path/filepath"
    "regexp"
    "strconv"
    "strings"
    "unicode/utf8"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func runeLen(s string) int64 {
    return int64(utf8.RuneCountInString(s))
}

func arrIdx(a []any, i int64) any {
    if i >= 0 && int(i) < len(a) { return a[i] }
    return nil
}

func arrSlice(a []any, start, end int64) []any {
    s, e := int(start), int(end)
    if s < 0 { s = len(a) + s }
    if e < 0 { e = len(a) + e }
    if s < 0 { s = 0 }
    if s > len(a) { s = len(a) }
    if e > len(a) { e = len(a) }
    if e < s { e = s }
    return a[s:e]
}

var fArgs []string

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1 { return os.Args[1:] }
    return nil
}

func paramAt(i int) string {
    a := argsList()
    if i >= 1 && i <= len(a) { return a[i-1] }
    return ""
}

func runCmd(c *exec.Cmd) int64 {
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    if err := c.Run(); err != nil {
        if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
        return 1
    }
    return 0
}

func capRun(script string, env []string) string {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func redirRun(script string, env []string) int64 {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    return runCmd(c)
}

func runPipe(stages [][]string) int64 {
    var cmds []*exec.Cmd
    for _, st := range stages {
        c := exec.Command(st[0], st[1:]...)
        c.Stderr = io.Discard
        cmds = append(cmds, c)
    }
    if len(cmds) == 0 { return 0 }
    for i := 0; i < len(cmds)-1; i++ {
        p, err := cmds[i].StdoutPipe()
        if err != nil { return 1 }
        cmds[i+1].Stdin = p
    }
    cmds[len(cmds)-1].Stdout = os.Stdout
    for i, c := range cmds {
        if i < len(cmds)-1 {
            if err := c.Start(); err != nil { return 1 }
        } else if err := c.Run(); err != nil {
            if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
            return 1
        }
    }
    for i := 0; i < len(cmds)-1; i++ { cmds[i].Wait() }
    return 0
}

func reMatch(pattern, s string) bool {
    ok, _ := regexp.MatchString(pattern, s)
    return ok
}

func globRe(pat string, greedy bool) string {
    var b strings.Builder
    for i := 0; i < len(pat); i++ {
        switch pat[i] {
        case '*':
            if greedy { b.WriteString(".*") } else { b.WriteString(".*?") }
        case '?': b.WriteString(".")
        case '.', '(', ')', '+', '|', '^', '$', '[', ']', '{', '}', '\\':
            b.WriteByte('\\'); b.WriteByte(pat[i])
        default: b.WriteByte(pat[i])
        }
    }
    return b.String()
}

func pStrip(shortest bool, v, pat string) string {
    re := regexp.MustCompile("^(?:" + globRe(pat, !shortest) + ")")
    loc := re.FindStringIndex(v)
    if loc == nil { return v }
    return v[loc[1]:]
}

func pSuf(shortest bool, v, pat string) string {
    var re *regexp.Regexp
    if shortest {
        re = regexp.MustCompile("^(.*)" + globRe(pat, true) + ".*?$")
    } else {
        re = regexp.MustCompile("^(.*?)" + globRe(pat, true) + ".*$")
    }
    m := re.FindStringSubmatchIndex(v)
    if m == nil { return v }
    return v[:m[3]]
}

func pRep(v, pat, rep string, all bool) string {
    re := regexp.MustCompile(globRe(pat, true))
    if all { return re.ReplaceAllString(v, rep) }
    l := re.FindStringIndex(v)
    if l == nil { return v }
    return v[:l[0]] + rep + v[l[1]:]
}

var vars = map[string]any{}

func pValRaw(name string) any {
    if v, ok := vars[name]; ok { return v }
    return os.Getenv(name)
}

func pVal(name string) string {
    if strings.HasPrefix(name, "#") {
        n := name[1:]
        if strings.HasSuffix(n, "[@]") {
            a, _ := pValRaw(n[:len(n)-3]).([]any)
            return strconv.FormatInt(int64(len(a)), 10)
        }
        if a, ok := pValRaw(n).([]any); ok {
            return strconv.FormatInt(int64(len(a)), 10)
        }
        return strconv.FormatInt(runeLen(s2s(pValRaw(n))), 10)
    }
    if strings.HasSuffix(name, "[@]") {
        a, _ := pValRaw(name[:len(name)-3]).([]any)
        out := make([]string, 0, len(a))
        for _, v := range a { out = append(out, s2s(v)) }
        return strings.Join(out, " ")
    }
    if i := strings.Index(name, "["); i > 0 && strings.HasSuffix(name, "]") {
        a, _ := pValRaw(name[:i]).([]any)
        k, err := strconv.ParseInt(name[i+1:len(name)-1], 0, 64)
        if err != nil {
            m, _ := pValRaw(name[:i]).(map[string]any)
            return s2s(m[name[i+1:len(name)-1]])
        }
        return s2s(arrIdx(a, k))
    }
    return s2s(pValRaw(name))
}

func pList(op, name string, args ...string) []string {
    if op == "@" || op == "*" { return argsList() }
    if strings.HasPrefix(name, "#") {
        return []string{pVal(name)}
    }
    if strings.HasSuffix(name, "[@]") {
        base := name[:len(name)-3]
        a, _ := pValRaw(base).([]any)
        if op == "slice" && len(args) >= 1 {
            st := int64(0)
            if len(args) > 0 { st, _ = strconv.ParseInt(args[0], 0, 64) }
            en := int64(len(a))
            if len(args) > 1 { en, _ = strconv.ParseInt(args[1], 0, 64) }
            a = arrSlice(a, st, st+en)
        }
        out := make([]string, 0, len(a))
        for _, v := range a { out = append(out, s2s(v)) }
        return out
    }
    return []string{pExp(op, name, args...)}
}

func pExp(op, name string, args ...string) string {
    v := pVal(name)
    switch op {
    case "":
        return v
    case "^":
        if v == "" { return v }
        return strings.ToUpper(v[:1]) + v[1:]
    case "^^":
        return strings.ToUpper(v)
    case ",":
        if v == "" { return v }
        return strings.ToLower(v[:1]) + v[1:]
    case ",,":
        return strings.ToLower(v)
    case "#":
        if len(args) > 0 { return pStrip(true, v, args[0]) }
        return v
    case "##":
        if len(args) > 0 { return pStrip(false, v, args[0]) }
        return v
    case "%":
        if len(args) > 0 { return pSuf(true, v, args[0]) }
        return v
    case "%%":
        if len(args) > 0 { return pSuf(false, v, args[0]) }
        return v
    case "/":
        if len(args) >= 2 { return pRep(v, args[0], args[1], false) }
        if len(args) >= 1 { return pRep(v, args[0], "", false) }
        return v
    case "//":
        if len(args) >= 2 { return pRep(v, args[0], args[1], true) }
        if len(args) >= 1 { return pRep(v, args[0], "", true) }
        return v
    case "len":
        return strconv.FormatInt(runeLen(v), 10)
    case "basename":
        return filepath.Base(v)
    case "dirname":
        return filepath.Dir(v)
    case "slice":
        st := int64(0)
        if len(args) > 0 { st, _ = strconv.ParseInt(args[0], 0, 64) }
        if st < 0 { st = int64(runeLen(v)) + st }
        if st > int64(runeLen(v)) { st = int64(runeLen(v)) }
        if st < 0 { st = 0 }
        rs := []rune(v)
        if len(args) < 2 { return string(rs[st:]) }
        ln, _ := strconv.ParseInt(args[1], 0, 64)
        en := st + ln
        if en < 0 { en = int64(len(rs)) + en }
        if en > int64(len(rs)) { en = int64(len(rs)) }
        if en < st { en = st }
        return string(rs[st:en])
    case ":-", "-":
        if v == "" && len(args) > 0 { return expand(args[0]) }
        return v
    case ":=":
        if v == "" && len(args) > 0 {
            d := expand(args[0])
            vars[name] = d
            return d
        }
        return v
    case ":?":
        if v == "" {
            if len(args) > 0 { _ = expand(args[0]) }
            os.Exit(1)
        }
        return v
    case "+":
        if v == "" { return "" }
        if len(args) > 0 { return expand(args[0]) }
        return v
    }
    return v
}

func expand(s string) string {
    var out strings.Builder
    for i := 0; i < len(s); {
        if s[i] == '$' && i+1 < len(s) {
            if s[i+1] == '(' {
                d := 1
                j := i + 2
                for j < len(s) && d > 0 {
                    if s[j] == '(' { d++ }
                    if s[j] == ')' { d-- }
                    j++
                }
                out.WriteString(capRun(s[i+2:j-1], nil))
                i = j
                continue
            }
            if s[i+1] == '{' {
                j := i + 2
                for j < len(s) && s[j] != '}' { j++ }
                if j >= len(s) { out.WriteString(s[i:]); break }
                out.WriteString(pExpStr(s[i+2 : j]))
                i = j + 1
                continue
            }
            j := i + 1
            for j < len(s) && (s[j] >= 'a' && s[j] <= 'z' || s[j] >= 'A' && s[j] <= 'Z' || s[j] >= '0' && s[j] <= '9' || s[j] == '_') { j++ }
            out.WriteString(pVal(s[i+1 : j]))
            i = j
            continue
        }
        out.WriteByte(s[i])
        i++
    }
    return out.String()
}

func pExpStr(inner string) string {
    for _, op := range []string{":-", ":=", ":?", "##", "%%", "//", "^^", ",,", "#", "%", "/", "^", ","} {
        if idx := strings.Index(inner, op); idx > 0 {
            return pExp(op, inner[:idx], inner[idx+len(op):])
        }
    }
    if strings.HasPrefix(inner, "#") { return pVal(inner) }
    return pVal(inner)
}

func bprintf(f string, a ...string) {
    fmt.Print(bprintfStr(f, a...))
}

func bprintfStr(f string, a ...string) string {
    f = bEsc(f)
    if len(a) == 0 { return f }
    var out strings.Builder
    for len(a) > 0 {
        var used int
        out.WriteString(bfmtOnce(f, a, &used))
        if used == 0 { break }
        a = a[used:]
    }
    return out.String()
}

func bfmtOnce(f string, a []string, used *int) string {
    var out strings.Builder
    for i := 0; i < len(f); i++ {
        c := f[i]
        if c != '%' || i+1 >= len(f) { out.WriteByte(c); continue }
        j := i + 1
        for j < len(f) && strings.ContainsRune("-+ #0", rune(f[j])) { j++ }
        for j < len(f) && f[j] >= '0' && f[j] <= '9' { j++ }
        if j < len(f) && f[j] == '.' {
            j++
            for j < len(f) && f[j] >= '0' && f[j] <= '9' { j++ }
        }
        for j < len(f) && strings.ContainsRune("lhLzjt", rune(f[j])) { j++ }
        if j >= len(f) { out.WriteByte('%'); break }
        v := f[j]
        spec := f[i : j+1]
        i = j
        switch v {
        case '%': out.WriteByte('%')
        case 's', 'q', 'd', 'i', 'u', 'o', 'x', 'X', 'e', 'E', 'f', 'F', 'g', 'G', 'c': {
            arg := ""
            if *used < len(a) { arg = a[*used] }
            *used++
            switch v {
            case 's': out.WriteString(bWidth(spec, arg))
            case 'q': out.WriteString(strconv.Quote(arg))
            case 'c': if len(arg) > 0 { out.WriteByte(arg[0]) }
            default: out.WriteString(bWidth(spec, bNum(v, arg)))
            }
        }
        case 'b': {
            arg := ""
            if *used < len(a) { arg = a[*used] }
            *used++
            out.WriteString(bEsc(arg))
        }
        default: out.WriteString(spec)
        }
    }
    return out.String()
}

func bWidth(spec, val string) string {
    d := spec[1:]
    minus := strings.HasPrefix(d, "-")
    if minus { d = d[1:] }
    w := 0
    for len(d) > 0 && d[0] >= '0' && d[0] <= '9' { w = w*10 + int(d[0]-'0'); d = d[1:] }
    if w <= len(val) { return val }
    pad := strings.Repeat(" ", w-len(val))
    if minus { return val + pad }
    return pad + val
}

func bNum(v byte, arg string) string {
    n, err := strconv.ParseInt(strings.TrimSpace(arg), 0, 64)
    if err != nil { n = 0 }
    switch v {
    case 'd', 'i', 'u': return strconv.FormatInt(n, 10)
    case 'o': return strconv.FormatInt(n, 8)
    case 'x': return strconv.FormatInt(n, 16)
    case 'X': return strings.ToUpper(strconv.FormatInt(n, 16))
    case 'e', 'E', 'f', 'F', 'g', 'G': {
        fl, err := strconv.ParseFloat(strings.TrimSpace(arg), 64)
        if err != nil { fl = 0 }
        if v == 'e' || v == 'E' { return strconv.FormatFloat(fl, 'e', -1, 64) }
        if v == 'f' || v == 'F' { return strconv.FormatFloat(fl, 'f', -1, 64) }
        return strconv.FormatFloat(fl, 'g', -1, 64)
    }
    }
    return arg
}

func bEsc(f string) string {
    var out strings.Builder
    for i := 0; i < len(f); i++ {
        c := f[i]
        if c != '\\' || i+1 >= len(f) { out.WriteByte(c); continue }
        i++
        switch f[i] {
        case 'n': out.WriteByte('\n')
        case 't': out.WriteByte('\t')
        case 'r': out.WriteByte('\r')
        case 'a': out.WriteByte('\a')
        case 'b': out.WriteByte('\b')
        case 'f': out.WriteByte('\f')
        case 'v': out.WriteByte('\v')
        case '\\': out.WriteByte('\\')
        case '\'': out.WriteByte('\'')
        case '"': out.WriteByte('"')
        case 'e': out.WriteByte(0x1b)
        case 'c': return out.String()
        case '0', '1', '2', '3', '4', '5', '6', '7': {
            v := int(f[i] - '0')
            for k := 0; k < 2 && i+1 < len(f) && f[i+1] >= '0' && f[i+1] <= '7'; k++ {
                i++; v = v*8 + int(f[i]-'0')
            }
            out.WriteByte(byte(v))
        }
        default: out.WriteByte('\\'); out.WriteByte(f[i])
        }
    }
    return out.String()
}
func main() {
    var arg any
    var desc any
    var rc any
    var rc_zero any
    var stderr any
    var stdout any
    var try func()
    var st int64

    try = func() {
        desc = paramAt(1); vars["desc"] = desc;
        st = 0;
        fmt.Println(fmt.Sprintf("==== %v ====", desc));
        st = 0;
        fmt.Print("  $ ");
        st = 0;
        t1 := pList("slice", "cmd[@]");
        for _, t2 := range t1 {
            arg = t2;
            vars["arg"] = arg;
            if reMatch(s2s("[[:space:]]"), s2s(arg)) {
                fmt.Print(fmt.Sprintf("'%v' ", arg));
                st = 0;
            } else {
                fmt.Print(fmt.Sprintf("%v ", arg));
                st = 0;
            }
        }
        fmt.Println("");
        st = 0;
        // declare stdout
        // declare stderr
        // declare rc
        // TODO(unsupported): capture
        stdout = 0; vars["stdout"] = stdout;
        st = 0;
        rc = st; vars["rc"] = rc;
        st = 0;
        stderr = capRun("( 'cat' \"/tmp/realpath_stderr.$$\" ) 2>/dev/null || 'true'", []string{"arg=" + s2s(arg), "desc=" + s2s(desc), "rc=" + s2s(rc), "rc_zero=" + s2s(rc_zero), "stderr=" + s2s(stderr), "stdout=" + s2s(stdout), "try=" + s2s(try)}); vars["stderr"] = stderr;
        st = 0;
        st = runCmd(exec.Command("rm", "-f", fmt.Sprintf("/tmp/realpath_stderr.%v", int64(os.Getpid()))));
        if (s2s(stdout) != "") {
            if (s2s(stdout) == s2s("*$'\\x00'*")) {
                fmt.Print("  stdout (NUL‑terminated): ");
                st = 0;
                st = redirRun("echo -n $stdout | 'od' '-A' 'n' '-t' 'x1z'", []string{"arg=" + s2s(arg), "desc=" + s2s(desc), "rc=" + s2s(rc), "rc_zero=" + s2s(rc_zero), "stderr=" + s2s(stderr), "stdout=" + s2s(stdout), "try=" + s2s(try)});
                fmt.Print("\n");
                st = 0;
            } else {
                fmt.Print(fmt.Sprintf("  stdout: %v\n", stdout));
                st = 0;
            }
        } else {
            fmt.Print("  stdout: (empty)\n");
            st = 0;
        }
        if (s2s(stderr) != "") {
            fmt.Print(fmt.Sprintf("  stderr: %v\n", stderr));
            st = 0;
        }
        bprintf("  exit code: %d\\n\\n", s2s(rc));
        st = 0;
    }
    fArgs = []string{"realpath (default, --physical) on a simple file", "realpath", "/bin"}; try(); fArgs = nil; // line 64
    fArgs = []string{"realpath on a two‑hop symlink chain: /usr/bin/vi → /etc/alternatives/vi → /usr/bin/vim.basic", "realpath", "/usr/bin/vi"}; try(); fArgs = nil; // line 67
    fArgs = []string{"realpath on a relative symlink: /usr/local/bin/pi", "realpath", "/usr/local/bin/pi"}; try(); fArgs = nil; // line 70
    fArgs = []string{"realpath on a regular file (no symlinks)", "realpath", "/etc/hostname"}; try(); fArgs = nil; // line 73
    fArgs = []string{"realpath on a directory with .. component", "realpath", "/tmp/.."}; try(); fArgs = nil; // line 76
    fArgs = []string{"--canonicalize-existing on an existing path", "realpath", "--canonicalize-existing", "/usr/bin/sh"}; try(); fArgs = nil; // line 85
    st = runCmd(exec.Command("try", "--canonicalize-existing on a path with a missing last component (should fail)", "realpath", "--canonicalize-existing", "/tmp/no_such_file_xyzzy")); // line 88
    if st != 0 {
        st = runCmd(exec.Command("true"));
    }
    fArgs = []string{"--canonicalize-missing on a path with a non‑existent leaf", "realpath", "--canonicalize-missing", "/tmp/no_such_file_xyzzy"}; try(); fArgs = nil; // line 96
    fArgs = []string{"--canonicalize-missing on a completely imaginary path", "realpath", "--canonicalize-missing", "/nonexistent/deeply/missing/file"}; try(); fArgs = nil; // line 99
    fArgs = []string{"--canonicalize-missing on a path with .. and non‑existent parts", "realpath", "--canonicalize-missing", "/tmp/../nonexistent/../foo"}; try(); fArgs = nil; // line 102
    fArgs = []string{"--logical: /bin/..  (bin is a symlink, logical resolves .. before following it)", "realpath", "--logical", "/bin/.."}; try(); fArgs = nil; // line 110
    fArgs = []string{"default (--physical) for comparison: /bin/..", "realpath", "--physical", "/bin/.."}; try(); fArgs = nil; // line 113
    fArgs = []string{"--physical: /bin/..  (explicit, same as default)", "realpath", "--physical", "/bin/.."}; try(); fArgs = nil; // line 121
    fArgs = []string{"--strip (no symlink expansion) on /usr/bin/vi", "realpath", "--strip", "/usr/bin/vi"}; try(); fArgs = nil; // line 129
    fArgs = []string{"--strip on /bin  (symlink /bin → usr/bin)", "realpath", "--strip", "/bin"}; try(); fArgs = nil; // line 132
    fArgs = []string{"compare: default (--physical) on /bin", "realpath", "/bin"}; try(); fArgs = nil; // line 135
    fArgs = []string{"--relative-to=/usr/bin for /usr/bin/sh", "realpath", "--relative-to=/usr/bin", "/usr/bin/sh"}; try(); fArgs = nil; // line 143
    fArgs = []string{"--relative-to=/tmp for /etc/hostname", "realpath", "--relative-to=/tmp", "/etc/hostname"}; try(); fArgs = nil; // line 146
    fArgs = []string{"--relative-to=/ for /etc/hostname", "realpath", "--relative-to=/", "/etc/hostname"}; try(); fArgs = nil; // line 149
    fArgs = []string{"--relative-base=/etc for /etc/hostname (below /etc → relative)", "realpath", "--relative-base=/etc", "/etc/hostname"}; try(); fArgs = nil; // line 157
    fArgs = []string{"--relative-base=/etc for /usr/bin/sh (not below /etc → absolute)", "realpath", "--relative-base=/etc", "/usr/bin/sh"}; try(); fArgs = nil; // line 160
    fArgs = []string{"--relative-base=/ with --relative-to=/tmp  (combined)", "realpath", "--relative-base=/", "--relative-to=/tmp", "/etc/hostname", "/usr/bin/sh"}; try(); fArgs = nil; // line 163
    fmt.Println("==== --zero with two paths ===="); // line 173
    st = 0;
    fmt.Print("  $ realpath --zero /bin /usr/bin/sh\n"); // line 174
    st = 0;
    st = runCmd(exec.Command("realpath", "--zero", "/bin", "/usr/bin/sh")); // line 175
    rc_zero = st; vars["rc_zero"] = rc_zero; // line 175
    st = 0;
    fmt.Print("  (raw output with NULs above; use od to verify):\n"); // line 177
    st = 0;
    fmt.Print("  "); // line 178
    st = 0;
    st = runPipe([][]string{[]string{"realpath", "--zero", "/bin", "/usr/bin/sh"}, []string{"od", "-A", "n", "-t", "x1z"}, []string{"head", "-3"}}); // line 178
    bprintf("  exit code: %d\\n\\n", s2s(rc_zero)); // line 179
    st = 0;
    st = runCmd(exec.Command("try", "--quiet suppresses error message for a truly invalid path", "realpath", "--quiet", "/nonexistent_dir_xyzzy/foo")); // line 188
    if st != 0 {
        st = runCmd(exec.Command("true"));
    }
    st = runCmd(exec.Command("try", "without --quiet for comparison (stderr appears)", "realpath", "/nonexistent_dir_xyzzy/foo")); // line 191
    if st != 0 {
        st = runCmd(exec.Command("true"));
    }
    fmt.Println("=== All tests completed ===");
    st = 0;
    return;
}
// 1 construct(s) lowered to TODO markers