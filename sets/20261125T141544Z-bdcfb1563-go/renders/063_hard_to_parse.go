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

func truthy(x any) bool {
    switch v := x.(type) {
    case string:
        return v != ""
    case bool:
        return v
    case int64:
        return v != 0
    case nil:
        return false
    }
    return true
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

func globMatch(pattern, s string) bool {
    ok, _ := filepath.Match(pattern, s)
    return ok
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

func main() {
    var __ps_tmp0 string
    var __ps_tmp1 string
    var __ps_tmp2 string
    var __sh2_casesubj_0 any
    var __sh2_casesubj_2 any
    var a any
    var args []any
    var array []any
    var array_i_ []any
    var b any
    var c any
    var complex_function func()
    var counter int64
    var d any
    var e any
    var f any
    var flags any
    var g any
    var h any
    var i int64
    var j int64
    var k any
    var key any
    var l any
    var m any
    var matrix []any
    var matrix_0_0_ any
    var matrix_1_1_ any
    var matrix_2_2_ any
    var matrix_3_3_ any
    var n any
    var options___key__ any
    var options____flags_j_1___ any
    var output string
    var prefix []any
    var result any
    var result_i_ any
    var value any
    var x any
    var y any
    var z any
    var st int64

    result = ((((((s2i(a) + s2i(b)) * (s2i(c) - s2i(d))) / (s2i(e) % s2i(f))) + powN(s2i(g),s2i(h))) - (s2i(i) << s2i(j))) | ((s2i(k) & s2i(l)) ^ (s2i(m) | s2i(n)))); vars["result"] = result; // line 7
    // declare matrix // line 10
    matrix_0_0_ = ((s2i(x) + s2i(y)) * s2i(z)); vars["matrix[0,0]"] = matrix_0_0_; // line 11
    matrix_1_1_ = s2s(arrIdx(array, s2i(os.Getenv("index")))); vars["matrix[1,1]"] = matrix_1_1_; // line 12
    matrix_2_2_ = strings.Join(pList("", "prefix"), " "); vars["matrix[2,2]"] = matrix_2_2_; // line 13
    matrix_3_3_ = s2s(arrLen(array)); vars["matrix[3,3]"] = matrix_3_3_; // line 14
    // TODO(unsupported): capture
    output = s2s(0); vars["output"] = output;
    fmt.Println(fmt.Sprintf("%v", pExp(":-", "var", "${default:-${fallback:-computed}}")));
    fmt.Println(fmt.Sprintf("%v", pExp(":-", "array[${index}]", "${default[@]:0:2}"))); // line 21
    fmt.Println(fmt.Sprintf("%v", pExp("slice", "!prefix*[@]", ""))); // line 22
    st = redirRun("( 'cat' ) 0<<'EOF'\n# This is a comment\n$(echo \"Command substitution\")\n${var:-default}\n$(( 1 + 2 * 3 ))\n\nEOF | 'grep' '-v' '^#' | 'sed' 's/^/  /'", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "__ps_tmp1=" + s2s(__ps_tmp1), "__ps_tmp2=" + s2s(__ps_tmp2), "__sh2_casesubj_0=" + s2s(__sh2_casesubj_0), "__sh2_casesubj_2=" + s2s(__sh2_casesubj_2), "a=" + s2s(a), "b=" + s2s(b), "c=" + s2s(c), "complex_function=" + s2s(complex_function), "counter=" + s2s(counter), "d=" + s2s(d), "e=" + s2s(e), "f=" + s2s(f), "g=" + s2s(g), "h=" + s2s(h), "i=" + s2s(i), "j=" + s2s(j), "k=" + s2s(k), "l=" + s2s(l), "m=" + s2s(m), "n=" + s2s(n), "output=" + s2s(output), "result=" + s2s(result), "x=" + s2s(x), "y=" + s2s(y), "z=" + s2s(z)});
    fmt.Println("All done");
    st = 0;
    if ((reMatch(s2s("^[0-9]+$"), s2s(os.Getenv("var"))) && (cmpN(s2i(os.Getenv("var")),0,">") != 0)) && fileTest("f", s2s(os.Getenv("file")))) {
        if (reMatch(s2s("value"), s2s(pExp("", "array[@]"))) || (cmpN(arrLen(array),5,">") != 0)) {
            if (s2s(capRun("echo \"$var\" | grep -q \"pattern\"", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "__ps_tmp1=" + s2s(__ps_tmp1), "__ps_tmp2=" + s2s(__ps_tmp2), "__sh2_casesubj_0=" + s2s(__sh2_casesubj_0), "__sh2_casesubj_2=" + s2s(__sh2_casesubj_2), "a=" + s2s(a), "b=" + s2s(b), "c=" + s2s(c), "complex_function=" + s2s(complex_function), "counter=" + s2s(counter), "d=" + s2s(d), "e=" + s2s(e), "f=" + s2s(f), "g=" + s2s(g), "h=" + s2s(h), "i=" + s2s(i), "j=" + s2s(j), "k=" + s2s(k), "l=" + s2s(l), "m=" + s2s(m), "n=" + s2s(n), "output=" + s2s(output), "result=" + s2s(result), "x=" + s2s(x), "y=" + s2s(y), "z=" + s2s(z)})) != "") {
                fmt.Println("Deeply nested condition met");
                st = 0;
            }
        }
    }
    switch {
        case globMatch("*[0-9]*", s2s(capRun("'echo' \"$var\" | 'tr' '[:upper:]' '[:lower:]'", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "__ps_tmp1=" + s2s(__ps_tmp1), "__ps_tmp2=" + s2s(__ps_tmp2), "__sh2_casesubj_0=" + s2s(__sh2_casesubj_0), "__sh2_casesubj_2=" + s2s(__sh2_casesubj_2), "a=" + s2s(a), "b=" + s2s(b), "c=" + s2s(c), "complex_function=" + s2s(complex_function), "counter=" + s2s(counter), "d=" + s2s(d), "e=" + s2s(e), "f=" + s2s(f), "g=" + s2s(g), "h=" + s2s(h), "i=" + s2s(i), "j=" + s2s(j), "k=" + s2s(k), "l=" + s2s(l), "m=" + s2s(m), "n=" + s2s(n), "output=" + s2s(output), "result=" + s2s(result), "x=" + s2s(x), "y=" + s2s(y), "z=" + s2s(z)}))):
            __sh2_casesubj_0 = fmt.Sprintf("%v", pExp(",,", "var")); vars["__sh2_casesubj_0"] = __sh2_casesubj_0;
            st = 0;
            if strings.Contains(s2s(__sh2_casesubj_0), "pattern") {
                fmt.Println("Double nested pattern");
                st = 0;
            } else {
                fmt.Println("Single nested pattern");
                st = 0;
            }
        case globMatch("*", s2s(capRun("'echo' \"$var\" | 'tr' '[:upper:]' '[:lower:]'", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "__ps_tmp1=" + s2s(__ps_tmp1), "__ps_tmp2=" + s2s(__ps_tmp2), "__sh2_casesubj_0=" + s2s(__sh2_casesubj_0), "__sh2_casesubj_2=" + s2s(__sh2_casesubj_2), "a=" + s2s(a), "b=" + s2s(b), "c=" + s2s(c), "complex_function=" + s2s(complex_function), "counter=" + s2s(counter), "d=" + s2s(d), "e=" + s2s(e), "f=" + s2s(f), "g=" + s2s(g), "h=" + s2s(h), "i=" + s2s(i), "j=" + s2s(j), "k=" + s2s(k), "l=" + s2s(l), "m=" + s2s(m), "n=" + s2s(n), "output=" + s2s(output), "result=" + s2s(result), "x=" + s2s(x), "y=" + s2s(y), "z=" + s2s(z)}))):
            fmt.Println("No numbers");
            st = 0;
        default:
            _ = 0;
    }
    complex_function = func() {
        i = "0"; vars["i"] = i;
        st = 0;
        for (cmpN(s2i(i),arrLen(args),"<") != 0) {
            __sh2_casesubj_2 = fmt.Sprintf("%v", pExp("", "args[i]")); vars["__sh2_casesubj_2"] = __sh2_casesubj_2;
            st = 0;
            // TODO(unsupported): strHasPrefix
            if truthy(0) {
                key = s2s(pExp("#", "args[i]", "--")); vars["key"] = key;
                st = 0;
                value = s2s(pExp(":-", "args[i+1]", "true")); vars["value"] = value;
                st = 0;
                options___key__ = value; vars["options[\"$key\"]"] = options___key__;
                st = 0;
                i += 2; vars["i"] = i;
                st = 0;
            // TODO(unsupported): strHasPrefix
            } else if truthy(0) {
                flags = s2s(pExp("#", "args[i]", "-")); vars["flags"] = flags;
                st = 0;
                j = "0"; vars["j"] = j;
                st = 0;
                for (cmpN(s2i(j),runeLen(s2i(flags)),"<") != 0) {
                    options____flags_j_1___ = "true"; vars["options[\"${flags:j:1}\"]"] = options____flags_j_1___;
                    st = 0;
                    j = j + 1; vars["j"] = j;
                    st = 0;
                }
                i = i + 1; vars["i"] = i;
                st = 0;
            } else {
                break;
            }
        }
        return;
    }
    i = 0; vars["i"] = i; // line 97
    st = 0;
    for (cmpN(s2i(i),arrLen(array),"<") != 0) {
        j = 0; vars["j"] = j;
        st = 0;
        for (cmpN(s2i(j),arrLen(array_i_),"<") != 0) {
            // TODO(unsupported): let-cond
            if 0 {
                // TODO(unsupported): arith
                result_i_ = 0; vars["result[i]"] = result_i_;
                st = 0;
            }
            j = j + 1; vars["j"] = j;
            st = 0;
        }
        i = i + 1; vars["i"] = i;
        st = 0;
    }
    // TODO(unsupported): capture
    __ps_tmp0 = s2s(0); vars["__ps_tmp0"] = __ps_tmp0;
    st = 0;
    // TODO(unsupported): Redirect
    // TODO(unsupported): Redirect
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp0", None)], capture: None, redirects: [], env: [] }
    st = redirRun("result=$(( ${var:-0} + ${array[${index:-0}]:-0} ))", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "__ps_tmp1=" + s2s(__ps_tmp1), "__ps_tmp2=" + s2s(__ps_tmp2), "__sh2_casesubj_0=" + s2s(__sh2_casesubj_0), "__sh2_casesubj_2=" + s2s(__sh2_casesubj_2), "a=" + s2s(a), "b=" + s2s(b), "c=" + s2s(c), "complex_function=" + s2s(complex_function), "counter=" + s2s(counter), "d=" + s2s(d), "e=" + s2s(e), "f=" + s2s(f), "flags=" + s2s(flags), "g=" + s2s(g), "h=" + s2s(h), "i=" + s2s(i), "j=" + s2s(j), "k=" + s2s(k), "key=" + s2s(key), "l=" + s2s(l), "m=" + s2s(m), "n=" + s2s(n), "output=" + s2s(output), "result=" + s2s(result), "value=" + s2s(value), "x=" + s2s(x), "y=" + s2s(y), "z=" + s2s(z)});
    st = redirRun("( ( ( 'echo' \"Level 3\"; ( 'echo' \"Level 4\"; 'echo' \"Still level 4\" ) ) | 'grep' \"Level\" ) | 'sed' 's/Level/Depth/' ) | 'wc' '-l'", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "__ps_tmp1=" + s2s(__ps_tmp1), "__ps_tmp2=" + s2s(__ps_tmp2), "__sh2_casesubj_0=" + s2s(__sh2_casesubj_0), "__sh2_casesubj_2=" + s2s(__sh2_casesubj_2), "a=" + s2s(a), "b=" + s2s(b), "c=" + s2s(c), "complex_function=" + s2s(complex_function), "counter=" + s2s(counter), "d=" + s2s(d), "e=" + s2s(e), "f=" + s2s(f), "flags=" + s2s(flags), "g=" + s2s(g), "h=" + s2s(h), "i=" + s2s(i), "j=" + s2s(j), "k=" + s2s(k), "key=" + s2s(key), "l=" + s2s(l), "m=" + s2s(m), "n=" + s2s(n), "output=" + s2s(output), "result=" + s2s(result), "value=" + s2s(value), "x=" + s2s(x), "y=" + s2s(y), "z=" + s2s(z)}); // line 130
    // TODO(unsupported): capture
    __ps_tmp1 = s2s(0); vars["__ps_tmp1"] = __ps_tmp1;
    st = 0;
    // TODO(unsupported): Redirect
    // TODO(unsupported): capture
    __ps_tmp2 = s2s(0); vars["__ps_tmp2"] = __ps_tmp2;
    st = 0;
    // TODO(unsupported): Redirect
    st = redirRun("( 'diff' \"$__ps_tmp1\" \"$__ps_tmp2\" ) 1>comparison.txt 2>&1", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "__ps_tmp1=" + s2s(__ps_tmp1), "__ps_tmp2=" + s2s(__ps_tmp2), "__sh2_casesubj_0=" + s2s(__sh2_casesubj_0), "__sh2_casesubj_2=" + s2s(__sh2_casesubj_2), "a=" + s2s(a), "b=" + s2s(b), "c=" + s2s(c), "complex_function=" + s2s(complex_function), "counter=" + s2s(counter), "d=" + s2s(d), "e=" + s2s(e), "f=" + s2s(f), "flags=" + s2s(flags), "g=" + s2s(g), "h=" + s2s(h), "i=" + s2s(i), "j=" + s2s(j), "k=" + s2s(k), "key=" + s2s(key), "l=" + s2s(l), "m=" + s2s(m), "n=" + s2s(n), "output=" + s2s(output), "result=" + s2s(result), "value=" + s2s(value), "x=" + s2s(x), "y=" + s2s(y), "z=" + s2s(z)});
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp1", None), Var("__ps_tmp2", None)], capture: None, redirects: [], env: [] }
    if ((((s2s(os.Getenv("var")) != "") && fileTest("f", s2s(os.Getenv("file")))) || fileTest("d", s2s(os.Getenv("dir")))) && (s2i(capRun("wc -l < \"$file\"", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "__ps_tmp1=" + s2s(__ps_tmp1), "__ps_tmp2=" + s2s(__ps_tmp2), "__sh2_casesubj_0=" + s2s(__sh2_casesubj_0), "__sh2_casesubj_2=" + s2s(__sh2_casesubj_2), "a=" + s2s(a), "b=" + s2s(b), "c=" + s2s(c), "complex_function=" + s2s(complex_function), "counter=" + s2s(counter), "d=" + s2s(d), "e=" + s2s(e), "f=" + s2s(f), "flags=" + s2s(flags), "g=" + s2s(g), "h=" + s2s(h), "i=" + s2s(i), "j=" + s2s(j), "k=" + s2s(k), "key=" + s2s(key), "l=" + s2s(l), "m=" + s2s(m), "n=" + s2s(n), "output=" + s2s(output), "result=" + s2s(result), "value=" + s2s(value), "x=" + s2s(x), "y=" + s2s(y), "z=" + s2s(z)})) > 10)) {
        fmt.Println("Complex test passed");
        st = 0;
    }
    fmt.Println("a1x a1y a1z a2x a2y a2z a3x a3y a3z b1x b1y b1z b2x b2y b2z b3x b3y b3z c1x c1y c1z c2x c2y c2z c3x c3y c3z");
    st = 0;
    // TODO(unsupported): Redirect // line 161
    fmt.Println(fmt.Sprintf("%v", complex_function(&[_]string{ "--long-option=value with spaces", "--array-option", "item1", "item2", "item3", "--flag", "positional argument", s2s(pExp(":-", "var", "default")), "computed" })));
    st = 0;
    // TODO(unsupported): pipeline // line 173
    st = 0;
    _ = __ps_tmp0
    _ = __ps_tmp1
    _ = __ps_tmp2
    _ = __sh2_casesubj_2
    _ = counter
    _ = key
    _ = matrix
    _ = matrix_0_0_
    _ = matrix_1_1_
    _ = matrix_2_2_
    _ = matrix_3_3_
    _ = options___key__
    _ = options____flags_j_1___
    _ = output
    _ = prefix
    _ = result
    _ = result_i_
    _ = st
    return;
}
// 16 construct(s) lowered to TODO markers