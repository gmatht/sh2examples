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

func setArrAt(a []any, i int64, v any) []any {
    for int64(len(a)) <= i { a = append(a, nil) }
    a[i] = v
    return a
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
    var arr any
    var assoc any
    var comb any
    var comb2 any
    var global_var any
    var lc any
    var myexport any
    var myfunc func()
    var n any
    var original any
    var plain any
    var printtest any
    var ref any
    var rovar any
    var set_global func()
    var singlearr []any
    var singlearr_0_ any
    var singlearr_1_ any
    var tracetest any
    var uc any
    var x any
    var st int64

    fmt.Println("=== typeset -i (integer attribute) ===");
    n = nil; // line 10
    n = "42"; vars["n"] = n; // line 11
    fmt.Println(fmt.Sprintf("After 'typeset -i n=42': n='%v'", n)); // line 12
    // TODO(unsupported): int
    n = 0; vars["n"] = n;
    fmt.Println(fmt.Sprintf("After 'n=n+1':          n='%v' (integer arithmetic applied)", n)); // line 14
    // TODO(unsupported): int
    n = 0; vars["n"] = n;
    fmt.Println(fmt.Sprintf("After 'n=\"hello\"':     n='%v' (assigns 0; non-numeric string becomes 0)", n)); // line 16
    fmt.Println(""); // line 17
    fmt.Println("=== typeset -r (readonly attribute) ===");
    rovar = nil; // line 21
    rovar = "immutable"; vars["rovar"] = rovar; // line 22
    fmt.Println(fmt.Sprintf("After 'typeset -r rovar=immutable': rovar='%v'", rovar)); // line 23
    fmt.Println("(Attempting 'rovar=change' would cause an error; skipped for safety.)");
    fmt.Println(""); // line 25
    fmt.Println("=== typeset -l (lowercase attribute) ===");
    lc = nil; // line 29
    lc = "HELLO WORLD"; vars["lc"] = lc; // line 30
    fmt.Println(fmt.Sprintf("After 'typeset -l lc=\"HELLO WORLD\"': lc='%v'", lc)); // line 31
    lc = "another test"; vars["lc"] = lc;
    fmt.Println(fmt.Sprintf("After 'lc=\"ANOTHER TEST\"':           lc='%v'", lc)); // line 33
    fmt.Println(""); // line 34
    fmt.Println("=== typeset -u (uppercase attribute) ===");
    uc = nil; // line 38
    uc = "hello world"; vars["uc"] = uc; // line 39
    fmt.Println(fmt.Sprintf("After 'typeset -u uc=\"hello world\"': uc='%v'", uc)); // line 40
    uc = "ANOTHER TEST"; vars["uc"] = uc;
    fmt.Println(fmt.Sprintf("After 'uc=\"another test\"':            uc='%v'", uc)); // line 42
    fmt.Println(""); // line 43
    fmt.Println("=== typeset -x (export attribute) ===");
    myexport = nil; // line 47
    myexport = "exported_value"; vars["myexport"] = myexport; // line 48
    fmt.Println("After 'typeset -x myexport=exported_value'");
    fmt.Println("Variable is exported:");
    st = runPipe([][]string{[]string{"env"}, []string{"grep", "^myexport="}});
    if st != 0 {
        st = runCmd(exec.Command("echo", "(myexport not found in env — possible scope issue)"));
    }
    fmt.Println(""); // line 53
    st = 0;
    fmt.Println("=== typeset -a (indexed array) ===");
    st = 0;
    arr = nil; // line 57
    fmt.Println(fmt.Sprintf("After 'typeset -a arr=(10 20 30)': arr=(%v)", strings.Join(pList("slice", "arr", "@", ""), " "))); // line 59
    st = 0;
    fmt.Println(fmt.Sprintf("arr[0]='%v' arr[1]='%v' arr[2]='%v'", pExp("", "arr[0]"), pExp("", "arr[1]"), pExp("", "arr[2]"))); // line 60
    st = 0;
    fmt.Println(""); // line 61
    st = 0;
    fmt.Println("=== typeset -A (associative array) ===");
    st = 0;
    assoc = nil; // line 65
    fmt.Println("After 'typeset -A assoc=([key1]=value1 [key2]=value2)'");
    st = 0;
    fmt.Println(fmt.Sprintf("assoc[key1]='%v'  assoc[key2]='%v'", pExp("", "assoc[key1]"), pExp("", "assoc[key2]"))); // line 68
    st = 0;
    fmt.Println(""); // line 69
    st = 0;
    fmt.Println("=== typeset -n (name reference) ===");
    st = 0;
    original = nil; // line 73
    ref = nil;
    original = "I am the original"; vars["original"] = original;
    st = 0;
    ref = "original"; vars["ref"] = ref; // line 75
    st = 0;
    fmt.Println("After 'typeset -n ref=original':");
    st = 0;
    fmt.Println(fmt.Sprintf("original='%v'", original)); // line 77
    st = 0;
    fmt.Println(fmt.Sprintf("ref='%v'", ref)); // line 78
    st = 0;
    ref = "Changed via ref"; vars["ref"] = ref;
    st = 0;
    fmt.Println("After 'ref=\"Changed via ref\"':");
    st = 0;
    fmt.Println(fmt.Sprintf("original='%v'", original)); // line 81
    st = 0;
    fmt.Println(fmt.Sprintf("ref='%v'", ref)); // line 82
    st = 0;
    st = redirRun("( 'unset' '-n' 'ref' ) 2>/dev/null", []string{"arr=" + s2s(arr), "assoc=" + s2s(assoc), "comb=" + s2s(comb), "comb2=" + s2s(comb2), "lc=" + s2s(lc), "myexport=" + s2s(myexport), "myfunc=" + s2s(myfunc), "n=" + s2s(n), "original=" + s2s(original), "ref=" + s2s(ref), "rovar=" + s2s(rovar), "set_global=" + s2s(set_global), "uc=" + s2s(uc), "x=" + s2s(x)}); // line 83
    fmt.Println(""); // line 84
    st = 0;
    fmt.Println("=== typeset -f (display function definition) ===");
    st = 0;
    myfunc = func() {
        fmt.Println("Inside myfunc");
        st = 0;
        x = "5"; vars["x"] = x;
        st = 0;
        fmt.Println(fmt.Sprintf("x=%v", x));
        st = 0;
    }
    fmt.Println("Output of 'typeset -f myfunc':");
    st = 0;
    // declare myfunc // line 94
    fmt.Println(""); // line 95
    st = 0;
    fmt.Println("=== typeset -F (list function names) ===");
    st = 0;
    // declare myfunc // line 99
    fmt.Println(""); // line 100
    st = 0;
    fmt.Println("=== typeset -g (global scope in function) ===");
    st = 0;
    set_global = func() { // line 104
        global_var = "I am global"; vars["global_var"] = global_var;
        st = 0;
    }
    fArgs = []string{}; set_global(); fArgs = nil; // line 107
    fmt.Println(fmt.Sprintf("After set_global(): global_var='%v'", global_var)); // line 108
    st = 0;
    fmt.Println(""); // line 109
    st = 0;
    fmt.Println("=== typeset -t (trace attribute) ===");
    st = 0;
    tracetest = "traced"; vars["tracetest"] = tracetest; // line 113
    st = 0;
    fmt.Println("After 'typeset -t tracetest=traced': (trace attribute set)");
    st = 0;
    fmt.Println(fmt.Sprintf("tracetest='%v'", tracetest)); // line 115
    st = 0;
    fmt.Println(""); // line 116
    st = 0;
    fmt.Println("=== typeset -p (print attribute info) ===");
    st = 0;
    printtest = nil; // line 120
    printtest = "99"; vars["printtest"] = printtest; // line 121
    st = 0;
    // declare printtest // line 122
    fmt.Println("After 'typeset -i -r printtest=99':");
    st = 0;
    // declare printtest // line 124
    fmt.Println(""); // line 125
    st = 0;
    fmt.Println("=== combined: typeset -il (integer + lowercase) ===");
    st = 0;
    comb = nil; // line 129
    comb = "42"; vars["comb"] = comb; // line 130
    st = 0;
    fmt.Println(fmt.Sprintf("After 'typeset -il comb=42': comb='%v' (integer + lowercase)", comb)); // line 131
    st = 0;
    // TODO(unsupported): lc
    comb = 0; vars["comb"] = comb;
    st = 0;
    fmt.Println(fmt.Sprintf("After 'comb=comb+1':    comb='%v' (integer arithmetic active)", comb)); // line 133
    st = 0;
    fmt.Println(""); // line 134
    st = 0;
    fmt.Println("=== combined: typeset -iu (integer + uppercase) ===");
    st = 0;
    comb2 = nil; // line 137
    comb2 = "99"; vars["comb2"] = comb2; // line 138
    st = 0;
    fmt.Println(fmt.Sprintf("After 'typeset -iu comb2=99': comb2='%v' (integer + uppercase)", comb2)); // line 139
    st = 0;
    // TODO(unsupported): uc
    comb2 = 0; vars["comb2"] = comb2;
    st = 0;
    fmt.Println(fmt.Sprintf("After 'comb2=comb2+1': comb2='%v' (integer arithmetic active)", comb2)); // line 141
    st = 0;
    fmt.Println(""); // line 142
    st = 0;
    fmt.Println("=== typeset -a (indexed array, individual element) ===");
    st = 0;
    singlearr = nil; // line 146
    // declare singlearr // line 147
    singlearr = setArrAt(singlearr, 0, "first"); vars["singlearr"] = singlearr;
    singlearr = setArrAt(singlearr, 1, "second"); vars["singlearr"] = singlearr;
    fmt.Println(fmt.Sprintf("singlearr[0]='%v'  singlearr[1]='%v'", pExp("", "singlearr[0]"), pExp("", "singlearr[1]"))); // line 150
    st = 0;
    fmt.Println(""); // line 151
    st = 0;
    fmt.Println("=== typeset (no flag) ===");
    st = 0;
    plain = nil; // line 155
    plain = "just a string"; vars["plain"] = plain; // line 156
    st = 0;
    fmt.Println(fmt.Sprintf("After 'typeset plain=\"just a string\"': plain='%v'", plain)); // line 157
    st = 0;
    fmt.Println(""); // line 158
    st = 0;
    fmt.Println("=== Demonstration complete. ===");
    st = 0;
    _ = arr
    _ = assoc
    _ = myexport
    _ = myfunc
    _ = printtest
    _ = singlearr
    _ = singlearr_0_
    _ = singlearr_1_
    _ = st
    return;
}
// 4 construct(s) lowered to TODO markers