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

func s2i(x any) int64 {
    s := strings.TrimSpace(fmt.Sprintf("%v", x))
    if n, err := strconv.ParseInt(s, 0, 64); err == nil { return n }
    return 0
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

func paramAt(i int) string {
    a := argsList()
    if i >= 1 && i <= len(a) { return a[i-1] }
    return ""
}

func capRun(script string, env []string) string {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
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

func main() {
    var array []any
    var complex_function func()
    var complex_var any
    var message string
    var param1 any
    var param2 any
    var param3 any
    var result int64
    var test_locals func()
    var var1 any
    var var2 any
    var var3 any
    var st int64

    fmt.Println("Testing ambiguous operators...");
    result = 512; vars["result"] = result;
    fmt.Println(fmt.Sprintf("2**3**2 = %v", result)); // line 10
    fmt.Println("Testing complex parameter expansions...");
    complex_var = "hello world"; vars["complex_var"] = complex_var;
    fmt.Println(fmt.Sprintf("%v", pExp("#", "complex_var", "*o"))); // line 15
    fmt.Println(fmt.Sprintf("%v", pExp("##", "complex_var", "*o"))); // line 16
    fmt.Println(fmt.Sprintf("%v", pExp("%", "complex_var", "o*"))); // line 17
    fmt.Println(fmt.Sprintf("%v", pExp("%%", "complex_var", "o*"))); // line 18
    fmt.Println("Testing complex here-documents...");
    fmt.Print("This is a test line\nThis is not a test line\nThis is another test line\n");
    fmt.Println("Testing nested arithmetic...");
    result = 40; vars["result"] = result;
    fmt.Println(fmt.Sprintf("Complex arithmetic: %v", result)); // line 31
    fmt.Println("Testing nested command substitution...");
    fmt.Println(fmt.Sprintf("Current dir: %v", pExp(":-", "PWD", "$(pwd)"))); // line 35
    fmt.Println(fmt.Sprintf("User: %v", pExp(":-", "USER", "$(whoami)"))); // line 36
    fmt.Println("Testing process substitution...");
    fmt.Println("Testing complex brace expansion...");
    fmt.Println("a1x a1y a1z a2x a2y a2z a3x a3y a3z b1x b1y b1z b2x b2y b2z b3x b3y b3z c1x c1y c1z c2x c2y c2z c3x c3y c3z");
    fmt.Println("Testing simple case patterns...");
    if (!(paramAt(1) != "test")) {
        fmt.Println("Matched test");
    } else {
        fmt.Println("Default case");
    }
    complex_function = func() {
        param1 = paramAt(1); vars["param1"] = param1;
        param2 = s2s(pExp(":-", "2", "default")); vars["param2"] = param2;
        param3 = s2s(pExp("//", "3", "\\\"", "\\\\\\\"")); vars["param3"] = param3;
        fmt.Println(fmt.Sprintf("Param1: %v", param1));
        fmt.Println(fmt.Sprintf("Param2: %v", param2));
        fmt.Println(fmt.Sprintf("Param3: %v", param3));
        result = s2i(s2s(strings.Join(strings.Fields(capRun("'echo' \"$param1\" | 'sed' 's/old/new/g'", []string{"complex_function=" + s2s(complex_function), "complex_var=" + s2s(complex_var), "message=" + s2s(message), "param1=" + s2s(param1), "param2=" + s2s(param2), "param3=" + s2s(param3), "result=" + s2s(result), "test_locals=" + s2s(test_locals)})), " "))); vars["result"] = result;
        fmt.Println(fmt.Sprintf("Result: %v", result));
    }
    fmt.Println("Testing simple pipeline...");
    st = runPipe([][]string{[]string{"ls", "-l", "-a"}, []string{"grep", "^d"}, []string{"head", "-5"}});
    fmt.Println("Testing mixed arithmetic...");
    st = 0;
    result = 776; vars["result"] = result;
    st = 0;
    fmt.Println(fmt.Sprintf("Mixed base result: %v", result)); // line 82
    st = 0;
    fmt.Println("Testing complex string interpolation...");
    st = 0;
    message = fmt.Sprintf("Hello, %v! Your home is %v", pExp(":-", "USER", "$(whoami)"), pExp(":-", "HOME", "$(echo ~)")); vars["message"] = message; // line 86
    st = 0;
    fmt.Println(fmt.Sprintf("%v", message)); // line 87
    st = 0;
    fmt.Println("Testing simple test expressions...");
    st = 0;
    if fileTest("f", s2s("file.txt")) {
        fmt.Println("File exists");
        st = 0;
    } else {
        fmt.Println("File does not exist");
        st = 0;
    }
    fmt.Println("Testing complex array operations...");
    st = 0;
    array = append(array, "item4"); vars["array"] = array;
    fmt.Println(fmt.Sprintf("Array: %v", strings.Join(pList("slice", "array", "@", ""), " "))); // line 101
    st = 0;
    fmt.Println(fmt.Sprintf("Length: %v", s2s(s2s(arrLen(array)))));
    st = 0;
    fmt.Println(fmt.Sprintf("First item: %v", pExp("", "array[0]"))); // line 103
    st = 0;
    test_locals = func() { // line 106
        var1 = paramAt(1); vars["var1"] = var1;
        st = 0;
        var2 = s2s(pExp(":-", "2", "default_value")); vars["var2"] = var2;
        st = 0;
        var3 = s2s(capRun("'echo' \"$var1\" | 'tr' '[:lower:]' '[:upper:]'", []string{"complex_function=" + s2s(complex_function), "complex_var=" + s2s(complex_var), "message=" + s2s(message), "param1=" + s2s(param1), "param2=" + s2s(param2), "param3=" + s2s(param3), "result=" + s2s(result), "test_locals=" + s2s(test_locals), "var1=" + s2s(var1), "var2=" + s2s(var2)})); vars["var3"] = var3;
        st = 0;
        fmt.Println(fmt.Sprintf("Var1: %v", var1));
        st = 0;
        fmt.Println(fmt.Sprintf("Var2: %v", var2));
        st = 0;
        fmt.Println(fmt.Sprintf("Var3: %v", var3));
        st = 0;
    }
    fArgs = []string{"test\"quote", "second_param", "third\"param"}; complex_function(); fArgs = nil;
    fArgs = []string{"hello", "world"}; test_locals(); fArgs = nil;
    _ = complex_var
    _ = st
    return;
}