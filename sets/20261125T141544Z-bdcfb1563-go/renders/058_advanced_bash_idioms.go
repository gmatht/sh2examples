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

func s2i(x any) int64 {
    s := strings.TrimSpace(fmt.Sprintf("%v", x))
    if n, err := strconv.ParseInt(s, 0, 64); err == nil { return n }
    return 0
}

func land(a, b int64) int64 {
    if a != 0 && b != 0 { return 1 }
    return 0
}

func lor(a, b int64) int64 {
    if a != 0 || b != 0 { return 1 }
    return 0
}

func cmpN(a, b int64, op string) int64 {
    switch op {
    case "==": if a == b { return 1 }
    case "!=": if a != b { return 1 }
    case "<": if a < b { return 1 }
    case ">": if a > b { return 1 }
    case "<=": if a <= b { return 1 }
    case ">=": if a >= b { return 1 }
    }
    return 0
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

func main() {
    var __sh2_casesubj_0 any
    var a int64
    var b int64
    var c int64
    var data_type any
    var dir_count string
    var file_count string
    var file_info string
    var host any
    var i int64
    var j int64
    var letter any
    var letters []any
    var matrix []any
    var matrix_0_0_ any
    var matrix_0_1_ any
    var matrix_0_2_ any
    var matrix_1_0_ any
    var matrix_1_1_ any
    var matrix_1_2_ any
    var matrix_2_0_ any
    var matrix_2_1_ any
    var matrix_2_2_ any
    var num any
    var numbers []any
    var port int64
    var process_data func()
    var result any
    var user any
    var value any

    fmt.Println("=== Advanced Bash Idioms Examples ===");
    fmt.Println(""); // line 7
    fmt.Println("1. Nested loops with conditional logic and array manipulation:");
    numbers = []any{"1", "2", "3", "4", "5"}; vars["numbers"] = numbers; // line 11
    letters = []any{"a", "b", "c", "d", "e"}; vars["letters"] = letters; // line 12
    t1 := pList("slice", "numbers[@]");
    for _, t2 := range t1 {
        num = t2;
        vars["num"] = num;
        t3 := pList("slice", "letters[@]");
        for _, t4 := range t3 {
            letter = t4;
            vars["letter"] = letter;
            if ((s2i(num) > 3) && (s2s(letter) != "c")) {
                fmt.Println(fmt.Sprintf("  Number %v with letter %v (filtered)", num, letter));
            }
        }
    }
    fmt.Println(""); // line 20
    fmt.Println("2. Function with nested case statements and parameter expansion:");
    process_data = func() {
        data_type = paramAt(1); vars["data_type"] = data_type;
        value = paramAt(2); vars["value"] = value;
        if (!(s2s(data_type) != "string")) {
            __sh2_casesubj_0 = fmt.Sprintf("%v", pExp(",,", "value")); vars["__sh2_casesubj_0"] = __sh2_casesubj_0;
            if ((!(s2s(__sh2_casesubj_0) != "hello")) || (!(s2s(__sh2_casesubj_0) != "hi"))) {
                fmt.Println(fmt.Sprintf("  Greeting detected: %v", value));
            } else if ((!(s2s(__sh2_casesubj_0) != "bye")) || (!(s2s(__sh2_casesubj_0) != "goodbye"))) {
                fmt.Println(fmt.Sprintf("  Farewell detected: %v", value));
            } else {
                fmt.Println(fmt.Sprintf("  Unknown string: %v", value));
            }
        } else if (!(s2s(data_type) != "number")) {
            if reMatch(s2s("^[0-9]+$"), s2s(value)) {
                if (cmpN((s2i(value) % 2),0,"==") != 0) {
                    fmt.Println(fmt.Sprintf("  Even number: %v", value));
                } else {
                    fmt.Println(fmt.Sprintf("  Odd number: %v", value));
                }
            } else {
                fmt.Println(fmt.Sprintf("  Invalid number: %v", value));
            }
        } else {
            fmt.Println(fmt.Sprintf("  Unknown data type: %v", data_type));
        }
    }
    fArgs = []string{"string", "Hello"}; process_data(); fArgs = nil;
    fArgs = []string{"string", "Bye"}; process_data(); fArgs = nil;
    fArgs = []string{"number", "42"}; process_data(); fArgs = nil;
    fArgs = []string{"number", "17"}; process_data(); fArgs = nil;
    fmt.Println(""); // line 63
    fmt.Println("3. Complex conditional with command substitution and arithmetic:");
    file_count = s2s(capRun("'find' '.' '-maxdepth' '1' '-type' 'f' | 'wc' '-l'", []string{"__sh2_casesubj_0=" + s2s(__sh2_casesubj_0), "a=" + s2s(a), "b=" + s2s(b), "c=" + s2s(c), "data_type=" + s2s(data_type), "dir_count=" + s2s(dir_count), "file_count=" + s2s(file_count), "file_info=" + s2s(file_info), "host=" + s2s(host), "i=" + s2s(i), "j=" + s2s(j), "letter=" + s2s(letter), "num=" + s2s(num), "port=" + s2s(port), "process_data=" + s2s(process_data), "result=" + s2s(result), "user=" + s2s(user), "value=" + s2s(value)})); vars["file_count"] = file_count; // line 67
    dir_count = s2s(capRun("'find' '.' '-maxdepth' '1' '-type' 'd' | 'wc' '-l'", []string{"__sh2_casesubj_0=" + s2s(__sh2_casesubj_0), "a=" + s2s(a), "b=" + s2s(b), "c=" + s2s(c), "data_type=" + s2s(data_type), "dir_count=" + s2s(dir_count), "file_count=" + s2s(file_count), "file_info=" + s2s(file_info), "host=" + s2s(host), "i=" + s2s(i), "j=" + s2s(j), "letter=" + s2s(letter), "num=" + s2s(num), "port=" + s2s(port), "process_data=" + s2s(process_data), "result=" + s2s(result), "user=" + s2s(user), "value=" + s2s(value)})); vars["dir_count"] = dir_count; // line 68
    if ((s2i(file_count) > 0) && (s2i(dir_count) > 1)) {
        if (cmpN(s2i(file_count),s2i(dir_count),">") != 0) {
            fmt.Println(fmt.Sprintf("  More files (%v) than directories (%v)", file_count, dir_count));
        } else {
            if (cmpN(s2i(file_count),s2i(dir_count),"==") != 0) {
                fmt.Println(fmt.Sprintf("  Equal count: %v files and %v directories", file_count, dir_count));
            } else {
                fmt.Println(fmt.Sprintf("  More directories (%v) than files (%v)", dir_count, file_count));
            }
        }
    } else {
        fmt.Println("  Insufficient items for comparison");
    }
    fmt.Println(""); // line 81
    fmt.Println("4. Nested here-documents with parameter expansion:");
    user = "admin"; vars["user"] = user;
    host = "localhost"; vars["host"] = host;
    port = 22; vars["port"] = port;
    fmt.Print("    SSH Configuration:\n    $(cat <<'INNER'\n        User: $user\n        Host: $host\n        Port: $port\n        Status: $(ping -c 1 $host >/dev/null 2>&1 && echo \"Online\" || echo \"Offline\")\nINNER\n    )\n");
    fmt.Println(""); // line 99
    fmt.Println("5. Array processing with nested loops and conditional logic:");
    // declare matrix // line 103
    matrix_0_0_ = "1"; vars["matrix[0,0]"] = matrix_0_0_; // line 104
    matrix_0_1_ = "2"; vars["matrix[0,1]"] = matrix_0_1_; // line 104
    matrix_0_2_ = "3"; vars["matrix[0,2]"] = matrix_0_2_; // line 104
    matrix_1_0_ = "4"; vars["matrix[1,0]"] = matrix_1_0_; // line 105
    matrix_1_1_ = "5"; vars["matrix[1,1]"] = matrix_1_1_; // line 105
    matrix_1_2_ = "6"; vars["matrix[1,2]"] = matrix_1_2_; // line 105
    matrix_2_0_ = "7"; vars["matrix[2,0]"] = matrix_2_0_; // line 106
    matrix_2_1_ = "8"; vars["matrix[2,1]"] = matrix_2_1_; // line 106
    matrix_2_2_ = "9"; vars["matrix[2,2]"] = matrix_2_2_; // line 106
    for _, i = range []int64{s2i("0"), s2i("1"), s2i("2")} {
        vars["i"] = i;
        for _, j = range []int64{s2i("0"), s2i("1"), s2i("2")} {
            vars["j"] = j;
            value = s2s(arrIdx(matrix, s2i(((s2s(i) + ",") + s2s(j))))); vars["value"] = value;
            if (s2i(value) > 5) {
                fmt.Print(fmt.Sprintf("  [%v] ", value));
            } else {
                fmt.Print(fmt.Sprintf("  %v ", value));
            }
        }
        fmt.Println("");
    }
    fmt.Println(""); // line 119
    fmt.Println("6. Process substitution with nested commands and error handling:");
    fmt.Println(fmt.Sprintf("  First word: %v", pExp("%%", "test_string", " *")));
    fmt.Println(fmt.Sprintf("  Last word: %v", pExp("##", "test_string", "* ")));
    fmt.Println(fmt.Sprintf("  Middle: %v", pExp("#", "test_string", "* ")));
    fmt.Println(fmt.Sprintf("  Middle: %v", pExp("%", "test_string", " *")));
    fmt.Println(fmt.Sprintf("  Uppercase: %v", pExp("^^", "test_string")));
    fmt.Println(fmt.Sprintf("  Lowercase: %v", pExp(",,", "test_string")));
    fmt.Println(fmt.Sprintf("  Capitalize: %v", pExp("^", "test_string")));
    fmt.Println("");
    fmt.Println("11. Complex arithmetic with nested expressions:");
    a = 10; vars["a"] = a;
    b = 5; vars["b"] = b;
    c = 3; vars["c"] = c;
    result = (((a + b) * c) - ((a % b) / c)); vars["result"] = result;
    fmt.Println("  Expression: (a + b) * c - (a %% b) / c");
    fmt.Println(fmt.Sprintf("  Values: a=%v, b=%v, c=%v", a, b, c));
    fmt.Println(fmt.Sprintf("  Result: %v", result));
    if (lor(land(cmpN(a,b,">"),cmpN(b,c,"<")),cmpN((a % 2),0,"==")) != 0) {
        fmt.Println("  Complex condition met: a > b AND (b < c OR a is even)");
    }
    fmt.Println("");
    fmt.Println("12. Nested command substitution with error handling:");
    fmt.Println(fmt.Sprintf("  Current directory: %v", capCmd(exec.Command("pwd"))));
    fmt.Println(fmt.Sprintf("  Parent directory: %v", capCmd(exec.Command("dirname", s2s(capCmd(exec.Command("pwd")))))));
    fmt.Println(fmt.Sprintf("  Home directory: %v", capCmd(exec.Command("dirname", s2s(capCmd(exec.Command("dirname", s2s(capCmd(exec.Command("pwd"))))))))));
    file_info = s2s(capRun("( 'stat' '-c' '%s %y' 'nonexistent_file' ) 2>/dev/null || 'echo' 'File not found'", []string{"__sh2_casesubj_0=" + s2s(__sh2_casesubj_0), "a=" + s2s(a), "b=" + s2s(b), "c=" + s2s(c), "data_type=" + s2s(data_type), "dir_count=" + s2s(dir_count), "file_count=" + s2s(file_count), "file_info=" + s2s(file_info), "host=" + s2s(host), "i=" + s2s(i), "j=" + s2s(j), "letter=" + s2s(letter), "num=" + s2s(num), "port=" + s2s(port), "process_data=" + s2s(process_data), "result=" + s2s(result), "user=" + s2s(user), "value=" + s2s(value)})); vars["file_info"] = file_info;
    fmt.Println(fmt.Sprintf("  File info: %v", file_info));
    fmt.Println("");
    fmt.Println("=== Advanced Bash Idioms Examples Complete ===");
    _ = host
    _ = letters
    _ = matrix_0_0_
    _ = matrix_0_1_
    _ = matrix_0_2_
    _ = matrix_1_0_
    _ = matrix_1_1_
    _ = matrix_1_2_
    _ = matrix_2_0_
    _ = matrix_2_1_
    _ = matrix_2_2_
    _ = numbers
    _ = port
    _ = user
    return;
}