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
    "syscall"
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

func capCmd(c *exec.Cmd) string {
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
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

func fileTest(op string, p string) bool {
    st, err := os.Stat(p)
    switch op {
    case "e", "a": return err == nil
    case "f": return err == nil && st.Mode().IsRegular()
    case "d": return err == nil && st.IsDir()
    case "s": return err == nil && st.Size() > 0
    case "L", "h":
        ls, lerr := os.Lstat(p)
        return lerr == nil && ls.Mode()&os.ModeSymlink != 0
    case "p": return err == nil && st.Mode()&os.ModeNamedPipe != 0
    case "b": return err == nil && st.Mode()&os.ModeDevice != 0 && st.Mode()&os.ModeCharDevice == 0
    case "c": return err == nil && st.Mode()&os.ModeCharDevice != 0
    case "S": return err == nil && st.Mode()&os.ModeSocket != 0
    case "g": return err == nil && st.Mode()&os.ModeSetgid != 0
    case "k": return err == nil && st.Mode()&os.ModeSticky != 0
    case "u": return err == nil && st.Mode()&os.ModeSetuid != 0
    case "N": return false
    case "O":
        if err != nil { return false }
        if sys, ok := st.Sys().(*syscall.Stat_t); ok { return int(sys.Uid) == os.Getuid() }
        return false
    case "G":
        if err != nil { return false }
        if sys, ok := st.Sys().(*syscall.Stat_t); ok { return int(sys.Gid) == os.Getgid() }
        return false
    case "r": return syscall.Access(p, 4) == nil
    case "w": return syscall.Access(p, 2) == nil
    case "x": return syscall.Access(p, 1) == nil
    }
    return false
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

func main() {
    var TTY_DEV string
    var capture func()
    var dev any
    var ec any
    var label any
    var se any
    var so any
    var tmp_stderr any
    var tmp_stdout any
    var st int64

    capture = func() {
        label = paramAt(1); vars["label"] = label;
        st = 0;
        // declare tmp_stdout
        // declare tmp_stderr
        tmp_stdout = capCmd(exec.Command("mktemp", "/tmp/tty_demo_stdout_XXXXXX")); vars["tmp_stdout"] = tmp_stdout;
        st = 0;
        tmp_stderr = capCmd(exec.Command("mktemp", "/tmp/tty_demo_stderr_XXXXXX")); vars["tmp_stderr"] = tmp_stderr;
        st = 0;
        // TODO(unsupported): Redirect
        ec = s2s(strings.Join(strings.Fields(s2s(st)), " ")); vars["ec"] = ec;
        st = 0;
        // declare so
        so = capCmd(exec.Command("cat", s2s(tmp_stdout))); vars["so"] = so;
        st = 0;
        // declare se
        se = capCmd(exec.Command("cat", s2s(tmp_stderr))); vars["se"] = se;
        st = 0;
        st = runCmd(exec.Command("rm", "-f", s2s(tmp_stdout), s2s(tmp_stderr)));
        fmt.Println(fmt.Sprintf("--- [%v] ---", pExp("", "label")));
        st = 0;
        fmt.Println(fmt.Sprintf("  cmd     : %v", strings.Join(argsList(), " ")));
        st = 0;
        fmt.Println(fmt.Sprintf("  exitcode: %v", pExp("", "ec")));
        st = 0;
        if (s2s(so) != "") {
            fmt.Println(fmt.Sprintf("  stdout  : %v", pExp("", "so")));
            st = 0;
        } else {
            fmt.Println("  stdout  : (empty)");
            st = 0;
        }
        if (s2s(se) != "") {
            fmt.Println(fmt.Sprintf("  stderr  : %v", pExp("", "se")));
            st = 0;
        }
        fmt.Println("");
        st = 0;
        return;
    }
    TTY_DEV = ""; vars["TTY_DEV"] = TTY_DEV;
    st = 0;
    for _, dev = range []any{"/dev/pts/2", "/dev/pts/3", "/dev/pts/4"} { // line 54
        vars["dev"] = dev;
        if fileTest("r", s2s(dev)) {
            TTY_DEV = s2s(dev); vars["TTY_DEV"] = TTY_DEV;
            st = 0;
            break;
        }
    }
    if (!(TTY_DEV != "")) {
        for _, dev = range []any{"\x01SH2GLOB\x01/dev/pts/*"} {
            vars["dev"] = dev;
            if fileTest("r", s2s(dev)) {
                TTY_DEV = s2s(dev); vars["TTY_DEV"] = TTY_DEV;
                st = 0;
                break;
            }
        }
    }
    fmt.Println(fmt.Sprintf("Using terminal device: %v", pExp(":-", "TTY_DEV", "NONE"))); // line 70
    st = 0;
    fmt.Println("");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 1: tty (default) — with a real terminal");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    if (TTY_DEV != "") {
        st = redirRun("( 'capture' '01-default-terminal' 'tty' ) 0<${TTY_DEV}", []string{"TTY_DEV=" + s2s(TTY_DEV), "capture=" + s2s(capture), "dev=" + s2s(dev), "ec=" + s2s(ec), "label=" + s2s(label), "se=" + s2s(se), "so=" + s2s(so), "tmp_stderr=" + s2s(tmp_stderr), "tmp_stdout=" + s2s(tmp_stdout)});
    } else {
        fmt.Println("  (skipped — no terminal device available)");
        st = 0;
        fmt.Println("");
        st = 0;
    }
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 2: tty (default) — stdin from /dev/null");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    st = redirRun("( 'capture' '02-not-a-tty' 'tty' ) 0</dev/null", []string{"TTY_DEV=" + s2s(TTY_DEV), "capture=" + s2s(capture), "dev=" + s2s(dev), "ec=" + s2s(ec), "label=" + s2s(label), "se=" + s2s(se), "so=" + s2s(so), "tmp_stderr=" + s2s(tmp_stderr), "tmp_stdout=" + s2s(tmp_stdout)});
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 3: tty (default) — piped input");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    fArgs = []string{"03-pipe-notty", "bash", "-c", "echo \"dummy\" | tty"}; capture(); fArgs = nil;
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 4: tty -s — with a real terminal (silent)");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    if (TTY_DEV != "") {
        st = redirRun("( 'capture' '04-silent-terminal' 'tty' '-s' ) 0<${TTY_DEV}", []string{"TTY_DEV=" + s2s(TTY_DEV), "capture=" + s2s(capture), "dev=" + s2s(dev), "ec=" + s2s(ec), "label=" + s2s(label), "se=" + s2s(se), "so=" + s2s(so), "tmp_stderr=" + s2s(tmp_stderr), "tmp_stdout=" + s2s(tmp_stdout)});
    } else {
        fmt.Println("  (skipped — no terminal device available)");
        st = 0;
        fmt.Println("");
        st = 0;
    }
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 5: tty -s — stdin from /dev/null (silent, not a tty)");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    st = redirRun("( 'capture' '05-silent-notty' 'tty' '-s' ) 0</dev/null", []string{"TTY_DEV=" + s2s(TTY_DEV), "capture=" + s2s(capture), "dev=" + s2s(dev), "ec=" + s2s(ec), "label=" + s2s(label), "se=" + s2s(se), "so=" + s2s(so), "tmp_stderr=" + s2s(tmp_stderr), "tmp_stdout=" + s2s(tmp_stdout)});
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 6: tty -s — piped input (silent, not a tty)");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    fArgs = []string{"06-silent-pipe", "bash", "-c", "echo \"dummy\" | tty -s"}; capture(); fArgs = nil;
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 7: tty --silent — long form, with a real terminal");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    if (TTY_DEV != "") {
        st = redirRun("( 'capture' '07-long-silent' 'tty' '--silent' ) 0<${TTY_DEV}", []string{"TTY_DEV=" + s2s(TTY_DEV), "capture=" + s2s(capture), "dev=" + s2s(dev), "ec=" + s2s(ec), "label=" + s2s(label), "se=" + s2s(se), "so=" + s2s(so), "tmp_stderr=" + s2s(tmp_stderr), "tmp_stdout=" + s2s(tmp_stdout)});
    } else {
        fmt.Println("  (skipped — no terminal device available)");
        st = 0;
        fmt.Println("");
        st = 0;
    }
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 8: tty --quiet — long form, with a real terminal");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    if (TTY_DEV != "") {
        st = redirRun("( 'capture' '08-long-quiet' 'tty' '--quiet' ) 0<${TTY_DEV}", []string{"TTY_DEV=" + s2s(TTY_DEV), "capture=" + s2s(capture), "dev=" + s2s(dev), "ec=" + s2s(ec), "label=" + s2s(label), "se=" + s2s(se), "so=" + s2s(so), "tmp_stderr=" + s2s(tmp_stderr), "tmp_stdout=" + s2s(tmp_stdout)});
    } else {
        fmt.Println("  (skipped — no terminal device available)");
        st = 0;
        fmt.Println("");
        st = 0;
    }
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 9: tty --silent — stdin from /dev/null (not a tty)");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    st = redirRun("( 'capture' '09-long-silent-notty' 'tty' '--silent' ) 0</dev/null", []string{"TTY_DEV=" + s2s(TTY_DEV), "capture=" + s2s(capture), "dev=" + s2s(dev), "ec=" + s2s(ec), "label=" + s2s(label), "se=" + s2s(se), "so=" + s2s(so), "tmp_stderr=" + s2s(tmp_stderr), "tmp_stdout=" + s2s(tmp_stdout)});
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 10: tty --quiet — stdin from /dev/null (not a tty)");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    st = redirRun("( 'capture' '10-long-quiet-notty' 'tty' '--quiet' ) 0</dev/null", []string{"TTY_DEV=" + s2s(TTY_DEV), "capture=" + s2s(capture), "dev=" + s2s(dev), "ec=" + s2s(ec), "label=" + s2s(label), "se=" + s2s(se), "so=" + s2s(so), "tmp_stderr=" + s2s(tmp_stderr), "tmp_stdout=" + s2s(tmp_stdout)});
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 11: tty (default) — inheriting stdin from the script");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    fArgs = []string{"11-inherited", "tty"}; capture(); fArgs = nil;
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 12: tty -s — inheriting stdin from the script");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    fArgs = []string{"12-inherited-silent", "tty", "-s"}; capture(); fArgs = nil;
    fmt.Println("============================================================");
    st = 0;
    fmt.Println(" SECTION 13: tty -s -s — repeated silent flag");
    st = 0;
    fmt.Println("============================================================");
    st = 0;
    if (TTY_DEV != "") {
        st = redirRun("( 'capture' '13-double-silent' 'tty' '-s' '-s' ) 0<${TTY_DEV}", []string{"TTY_DEV=" + s2s(TTY_DEV), "capture=" + s2s(capture), "dev=" + s2s(dev), "ec=" + s2s(ec), "label=" + s2s(label), "se=" + s2s(se), "so=" + s2s(so), "tmp_stderr=" + s2s(tmp_stderr), "tmp_stdout=" + s2s(tmp_stdout)});
    } else {
        fmt.Println("  (skipped — no terminal device available)");
        st = 0;
        fmt.Println("");
        st = 0;
    }
    fmt.Println("");
    st = 0;
    fmt.Println("All tty demo sections completed.");
    st = 0;
    _ = ec
    _ = label
    return;
}
// 1 construct(s) lowered to TODO markers