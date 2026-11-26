package main

import (
    "bufio"
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

func b2i(b bool) int64 {
    if b { return 1 }
    return 0
}

func powN(a, b int64) int64 {
    r := int64(1)
    for i := int64(0); i < b; i++ { r *= a }
    return r
}

func runeLen(s string) int64 {
    return int64(utf8.RuneCountInString(s))
}

func arrLen(a []any) int64 { return int64(len(a)) }

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
    var __cat_line1 string
    var aa []any
    var aa_one_ any
    var aa_three_ any
    var aa_two_ any
    var arr []any
    var ax int64
    var el any
    var k any
    var ly int64
    var y any
    var z any
    var st int64

    _ = os.Chdir("/tmp"); // line 14
    st = 0;
    st = b2i(!((5 > 3))); // line 17
    fmt.Println(fmt.Sprintf("arith:  %v", st)); // line 17
    st = 0;
    st = b2i(!((s2s("abc") == s2s("abc")))); // line 18
    fmt.Println(fmt.Sprintf("str:    %v", st)); // line 18
    st = 0;
    st = b2i(!(fileTest("f", s2s("/etc/passwd")))); // line 19
    fmt.Println(fmt.Sprintf("file:   %v", st)); // line 19
    st = 0;
    st = b2i(!(reMatch(s2s("^hello"), s2s("hello world")))); // line 20
    fmt.Println(fmt.Sprintf("regex:  %v", st)); // line 20
    st = 0;
    st = b2i(!((s2s("") == ""))); // line 21
    fmt.Println(fmt.Sprintf("empty:  %v", st)); // line 21
    st = 0;
    st = b2i(!((s2s("x") != ""))); // line 22
    fmt.Println(fmt.Sprintf("nonempty: %v", st)); // line 22
    st = 0;
    st = b2i(!((redirRun("[ \"a\"<\"b\" ]", []string{"ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)}) == 0))); // line 23
    fmt.Println(fmt.Sprintf("lt:     %v", st)); // line 23
    st = 0;
    st = b2i(!((s2s("a") != s2s("b")))); // line 24
    fmt.Println(fmt.Sprintf("ne:    %v", st)); // line 24
    st = 0;
    st = b2i(!((redirRun("[ !-e /no/such/file ]", []string{"ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)}) == 0))); // line 25
    fmt.Println(fmt.Sprintf("notfile: %v", st)); // line 25
    st = 0;
    fmt.Println(fmt.Sprintf("up=%v", "HELLO WORLD"));
    st = 0;
    fmt.Println(fmt.Sprintf("down=%v", "hello world"));
    st = 0;
    fmt.Println(fmt.Sprintf("firstup=%v", "HeLLo WoRLD"));
    st = 0;
    y = "one-two-one"; vars["y"] = y;
    st = 0;
    fmt.Println(fmt.Sprintf("gsub=%v", "1-two-1"));
    st = 0;
    fmt.Println(fmt.Sprintf("sub=%v", "1-two-one"));
    st = 0;
    fmt.Println(fmt.Sprintf("prefix=%v", "two-one"));
    st = 0;
    fmt.Println(fmt.Sprintf("suffix=%v", "one-two"));
    st = 0;
    fmt.Println(fmt.Sprintf("shortest=%v", pExp("##", "y", "*o"))); // line 40
    st = 0;
    fmt.Println(fmt.Sprintf("greedy=%v", pExp("%%", "y", "-*"))); // line 41
    st = 0;
    z = "0123456789"; vars["z"] = z;
    st = 0;
    fmt.Println(fmt.Sprintf("mid=%v", s2s("234")));
    st = 0;
    fmt.Println(fmt.Sprintf("tail=%v", strings.Join(pList("slice", "z", "5", ""), " "))); // line 46
    st = 0;
    fmt.Println(fmt.Sprintf("neg=%v", strings.Join(pList("slice", "z", " -3", ""), " "))); // line 47
    st = 0;
    fmt.Println(fmt.Sprintf("len=%v", "10"));
    st = 0;
    arr = []any{"alpha", "beta", "gamma"}; vars["arr"] = arr; // line 52
    fmt.Println(fmt.Sprintf("idx=%v", pExp("", "arr[1]"))); // line 53
    st = 0;
    fmt.Println(fmt.Sprintf("count=%v", s2s(s2s(arrLen(arr)))));
    st = 0;
    fmt.Println(fmt.Sprintf("all=%v", strings.Join(pList("slice", "arr", "@", ""), " "))); // line 55
    st = 0;
    t1 := pList("slice", "arr[@]"); // line 56
    for _, t2 := range t1 {
        el = t2;
        vars["el"] = el;
        fmt.Println(fmt.Sprintf("el=%v", el));
        st = 0;
    }
    // declare aa // line 60
    aa_one_ = "1"; vars["aa[one]"] = aa_one_; // line 61
    st = 0;
    aa_two_ = "2"; vars["aa[two]"] = aa_two_; // line 62
    st = 0;
    aa_three_ = "3"; vars["aa[three]"] = aa_three_; // line 63
    st = 0;
    fmt.Println(fmt.Sprintf("aa-one=%v", pExp("", "aa[one]"))); // line 64
    st = 0;
    t3 := strings.Fields(capRun("'echo' \"${!aaslice@}\" | 'tr' ' ' '\\n' | 'sort'", []string{"ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)})); // line 65
    for _, t4 := range t3 {
        k = t4;
        vars["k"] = k;
        fmt.Println(fmt.Sprintf("aa[%v]=%v", k, pExp("", "aa[$k]")));
        st = 0;
    }
    // TODO(unsupported): Redirect // line 68
    st = redirRun("( echo both ) 2>/tmp/gnu_both.txt", []string{"ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)});
    _ff0, _err0 := os.Open("/tmp/gnu_both.txt")
    if _err0 != nil { st = 1 } else {
        _sc0 := bufio.NewScanner(_ff0)
        for _sc0.Scan() {
            __cat_line1 = _sc0.Text()
            _ = __cat_line1
            fmt.Println(fmt.Sprintf("%v", __cat_line1));
            st = 0;
        }
    }
    st = redirRun("( echo err-to-stderr ) 1>&2", []string{"__cat_line1=" + s2s(__cat_line1), "ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)});
    ax = 8; vars["ax"] = ax;
    st = 0;
    fmt.Println(fmt.Sprintf("arith=%v", ax)); // line 79
    st = 0;
    ly = (ax * 2); vars["ly"] = ly;
    st = 0;
    fmt.Println(fmt.Sprintf("let=%v", ly)); // line 81
    st = 0;
    ax = ax + 1; vars["ax"] = ax;
    st = 0;
    fmt.Println(fmt.Sprintf("postfix=%v", ax)); // line 83
    st = 0;
    fmt.Println(fmt.Sprintf("pow=%v", powN(2,10))); // line 84
    st = 0;
    st = runPipe([][]string{[]string{"false"}, []string{"true"}});
    fmt.Println(fmt.Sprintf("ps=%v,%v", pExp("", "PIPESTATUS[0]"), pExp("", "PIPESTATUS[1]"))); // line 88
    st = 0;
    fmt.Print("no-nl");
    st = 0;
    fmt.Println(".");
    st = 0;
    fmt.Println("tab\there");
    st = 0;
    bprintf("-v", "pv", "value-%d", "42");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", os.Getenv("pv"))); // line 96
    st = 0;
    fmt.Println(fmt.Sprintf("seq: %v", capRun("'seq' '1' '3' | 'tr' '\\n' ' '", []string{"__cat_line1=" + s2s(__cat_line1), "ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)}))); // line 99
    st = 0;
    st = runCmd(exec.Command("ln", "-s", "-f", "/etc/passwd", "/tmp/gnu_link")); // line 100
    fmt.Println(fmt.Sprintf("readlink-f: %v", capCmd(exec.Command("readlink", "-f", "/tmp/gnu_link")))); // line 101
    st = 0;
    fmt.Println(fmt.Sprintf("readlink-e: %v", capCmd(exec.Command("readlink", "-e", "/tmp/gnu_link")))); // line 102
    st = 0;
    st = redirRun("( 'readlink' '-m' '/no/such/path' ) 1>/dev/null", []string{"__cat_line1=" + s2s(__cat_line1), "ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)}); // line 103
    fmt.Println(fmt.Sprintf("readlink-m rc=%v", st)); // line 103
    st = 0;
    os.WriteFile("/tmp/gnu_a.txt", []byte("abc\n"), 0644); // line 105
    os.WriteFile("/tmp/gnu_b.txt", []byte("abd\n"), 0644); // line 106
    st = runCmd(exec.Command("cmp", "-b", "/tmp/gnu_a.txt", "/tmp/gnu_b.txt")); // line 107
    fmt.Println(fmt.Sprintf("cmp-b rc=%v", st)); // line 107
    st = 0;
    st = runCmd(exec.Command("cmp", "-n", "2", "/tmp/gnu_a.txt", "/tmp/gnu_b.txt")); // line 108
    fmt.Println(fmt.Sprintf("cmp-n rc=%v", st)); // line 108
    st = 0;
    st = runCmd(exec.Command("cmp", "-i", "2", "/tmp/gnu_a.txt", "/tmp/gnu_b.txt")); // line 109
    fmt.Println(fmt.Sprintf("cmp-i rc=%v", st)); // line 109
    st = 0;
    st = redirRun("echo -n foo\nbar123\n | ( 'grep' '-E' '[0-9]+' ) >/dev/null", []string{"__cat_line1=" + s2s(__cat_line1), "ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)});
    fmt.Println(fmt.Sprintf("grep-P rc=%v", st)); // line 111
    st = 0;
    st = redirRun("'echo' 'a1b2c3' | 'sed' '-r' 's/[0-9]+/N/g'", []string{"__cat_line1=" + s2s(__cat_line1), "ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)});
    st = redirRun("'echo' 'AbCdEf' | 'tr' '[:lower:]' '[:upper:]'", []string{"__cat_line1=" + s2s(__cat_line1), "ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)});
    st = redirRun("'echo' '  spaced  ' | 'tr' '-s' ' '", []string{"__cat_line1=" + s2s(__cat_line1), "ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)});
    st = redirRun("echo -n 10K\n2M\n500\n | 'sort' '-h'", []string{"__cat_line1=" + s2s(__cat_line1), "ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)}); // line 115
    st = redirRun("echo -n short\nvery-long-line-here\n | 'wc' '-L'", []string{"__cat_line1=" + s2s(__cat_line1), "ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)});
    st = redirRun("echo -n abcdef | 'head' '-c' '3'", []string{"__cat_line1=" + s2s(__cat_line1), "ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)}); // line 117
    fmt.Println(""); // line 117
    st = 0;
    st = redirRun("echo -n abcdef | 'tail' '-c' '2'", []string{"__cat_line1=" + s2s(__cat_line1), "ax=" + s2s(ax), "el=" + s2s(el), "k=" + s2s(k), "ly=" + s2s(ly), "y=" + s2s(y), "z=" + s2s(z)});
    fmt.Println(""); // line 118
    st = 0;
    st = b2i(!(strings.EqualFold(s2s("ABC"), s2s("abc"))));
    if st == 0 {
        st = runCmd(exec.Command("echo", "nocase-match"));
    }
    st = b2i(!(strings.EqualFold(s2s("ABC"), s2s("abc"))));
    if st == 0 {
        st = runCmd(exec.Command("echo", "case-matters"));
    }
    if st != 0 {
        st = runCmd(exec.Command("echo", "case-sensitive"));
    }
    st = runCmd(exec.Command("rm", "-f", "/tmp/gnu_both.txt", "/tmp/gnu_link", "/tmp/gnu_a.txt", "/tmp/gnu_b.txt")); // line 127
    _ = aa
    _ = aa_one_
    _ = aa_three_
    _ = aa_two_
    _ = y
    _ = z
    return;
}
// 1 construct(s) lowered to TODO markers