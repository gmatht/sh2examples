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

func main() {
    var count string
    var d string
    var file any
    var files []any
    var get_file_size func()
    var here_string_result string
    var nested_result string
    var perl_result string
    var process_result string
    var size any
    var st int64

    d = s2s(capCmd(exec.Command("mktemp", "-d"))); vars["d"] = d; // line 4
    st = runCmd(exec.Command("cd", d)); // line 5
    if st != 0 {
        st = runCmd(exec.Command("exit", "1"));
    }
    fmt.Println("=== Complex Backtick Examples ===");
    st = 0;
    // TODO(unsupported): capture // line 11
    nested_result = s2s(0); vars["nested_result"] = nested_result;
    st = 0;
    fmt.Println(fmt.Sprintf("Nested backticks: %v", nested_result)); // line 12
    st = 0;
    count = s2s(capRun("'ls' '-1' | 'wc' '-l'", []string{"count=" + s2s(count), "d=" + s2s(d), "file=" + s2s(file), "get_file_size=" + s2s(get_file_size), "here_string_result=" + s2s(here_string_result), "nested_result=" + s2s(nested_result), "perl_result=" + s2s(perl_result), "process_result=" + s2s(process_result)})); vars["count"] = count; // line 15
    st = 0;
    fmt.Println(fmt.Sprintf("File count: %v", count)); // line 16
    st = 0;
    if (s2s("root") == s2s("root")) {
        fmt.Println("Running as root");
        st = 0;
    } else {
        fmt.Println("Not running as root");
        st = 0;
    }
    if false {
        fmt.Println("Running on Linux");
        st = 0;
    } else if true {
        fmt.Println("Running on macOS");
        st = 0;
    } else {
        fmt.Println("Running on other system");
        st = 0;
    }
    get_file_size = func() { // line 41
        file = paramAt(1); vars["file"] = file;
        st = 0;
        size = s2s(strings.Join(strings.Fields(capRun("( 'wc' '-c' ) 0<${file}", []string{"count=" + s2s(count), "d=" + s2s(d), "file=" + s2s(file), "get_file_size=" + s2s(get_file_size), "here_string_result=" + s2s(here_string_result), "nested_result=" + s2s(nested_result), "perl_result=" + s2s(perl_result), "process_result=" + s2s(process_result)})), " ")); vars["size"] = size;
        st = 0;
        fmt.Println(fmt.Sprintf("File %v has %v bytes", file, size));
        st = 0;
    }
    fArgs = []string{"000__01_file_directory_operations.sh"}; get_file_size(); fArgs = nil; // line 47
    files = []any{strings.Join(strings.Fields(capRun("( 'ls' '-1' '\x01SH2GLOB\x01*.sh' '\x01SH2GLOB\x01examples/*.sh' ) 2>/dev/null", []string{"count=" + s2s(count), "d=" + s2s(d), "file=" + s2s(file), "get_file_size=" + s2s(get_file_size), "here_string_result=" + s2s(here_string_result), "nested_result=" + s2s(nested_result), "perl_result=" + s2s(perl_result), "process_result=" + s2s(process_result), "size=" + s2s(size)})), " ")}; vars["files"] = files; // line 50
    fmt.Println(fmt.Sprintf("Shell scripts found: %v", s2s(s2s(arrLen(files)))));
    st = 0;
    t1 := pList("slice", "files[@]"); // line 52
    for _, t2 := range t1 {
        file = t2;
        vars["file"] = file;
        fmt.Println(fmt.Sprintf("  - %v", file));
        st = 0;
    }
    os.WriteFile("file1.txt", []byte("apple\nbanana\ncherry\n"), 0644);
    os.WriteFile("file2.txt", []byte("banana\ncherry\ndate\n"), 0644);
    // TODO(unsupported): capture
    process_result = s2s(0); vars["process_result"] = process_result;
    st = 0;
    fmt.Println("Process substitution result:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", process_result)); // line 61
    st = 0;
    // TODO(unsupported): capture // line 64
    here_string_result = s2s(0); vars["here_string_result"] = here_string_result;
    st = 0;
    fmt.Println(fmt.Sprintf("Here string result: %v", here_string_result)); // line 65
    st = 0;
    perl_result = s2s(capCmd(exec.Command("perl", "-e", "print \"Hello from Perl\\n\""))); vars["perl_result"] = perl_result; // line 69
    st = 0;
    fmt.Println(fmt.Sprintf("Perl result: %v", perl_result)); // line 70
    st = 0;
    st = runCmd(exec.Command("rm", "-f", "file1.txt", "file2.txt")); // line 73
    fmt.Println("=== Complex Backtick Examples Complete ===");
    st = 0;
    _ = os.Chdir("/"); // line 76
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 77
    _ = st
    return;
}
// 3 construct(s) lowered to TODO markers