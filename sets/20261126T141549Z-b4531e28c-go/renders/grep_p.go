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
    var ADAPTER string
    var ADAPTER3 string
    var BGPID any
    var CPID string
    var DEFAULT_ADAPTER string
    var DOINGS string
    var ERRORS string
    var N string
    var st int64

    _ = os.Chdir("/tmp"); // line 32
    os.WriteFile("grep_p_routes.txt", []byte("default via 10.0.0.1 dev eth0 proto dhcp src 10.0.0.1 metric 100\n"), 0644); // line 35
    os.WriteFile("grep_p_routes2.txt", []byte("default via 10.7.7.7 dev vpn_se proto static\n"), 0644); // line 36
    fmt.Println("-- functions.sh / softy: lookbehind -o extraction --");
    DEFAULT_ADAPTER = s2s(capRun("'grep' 'default' 'grep_p_routes.txt' | 'tail' '-1' | 'grep' '-Eo' 'dev ([^[:space:]]+)' | 'sed' 's/^dev //'", []string{"ADAPTER=" + s2s(ADAPTER), "ADAPTER3=" + s2s(ADAPTER3), "BGPID=" + s2s(BGPID), "CPID=" + s2s(CPID), "DEFAULT_ADAPTER=" + s2s(DEFAULT_ADAPTER), "DOINGS=" + s2s(DOINGS), "ERRORS=" + s2s(ERRORS), "N=" + s2s(N)})); vars["DEFAULT_ADAPTER"] = DEFAULT_ADAPTER;
    fmt.Println(fmt.Sprintf("adapter=[%v]", DEFAULT_ADAPTER)); // line 40
    fmt.Println("-- jobs.sh: lookbehind -o + grep -v + head -1 --");
    ADAPTER = s2s(capRun("'grep' 'default' 'grep_p_routes2.txt' | 'grep' '-Eo' 'dev ([^[:space:]]+)' | 'sed' 's/^dev //' | 'grep' '-v' 'vpn_se' | 'head' '-1'", []string{"ADAPTER=" + s2s(ADAPTER), "ADAPTER3=" + s2s(ADAPTER3), "BGPID=" + s2s(BGPID), "CPID=" + s2s(CPID), "DEFAULT_ADAPTER=" + s2s(DEFAULT_ADAPTER), "DOINGS=" + s2s(DOINGS), "ERRORS=" + s2s(ERRORS), "N=" + s2s(N)})); vars["ADAPTER"] = ADAPTER;
    fmt.Println(fmt.Sprintf("adapter2=[%v]", pExp(":-", "ADAPTER", "EMPTY"))); // line 44
    ADAPTER3 = s2s(capRun("'grep' 'default' 'grep_p_routes.txt' 'grep_p_routes2.txt' | 'grep' '-Eo' 'dev ([^[:space:]]+)' | 'sed' 's/^dev //' | 'grep' '-v' 'vpn_se' | 'head' '-1'", []string{"ADAPTER=" + s2s(ADAPTER), "ADAPTER3=" + s2s(ADAPTER3), "BGPID=" + s2s(BGPID), "CPID=" + s2s(CPID), "DEFAULT_ADAPTER=" + s2s(DEFAULT_ADAPTER), "DOINGS=" + s2s(DOINGS), "ERRORS=" + s2s(ERRORS), "N=" + s2s(N)})); vars["ADAPTER3"] = ADAPTER3;
    fmt.Println(fmt.Sprintf("adapter3=[%v]", pExp(":-", "ADAPTER3", "EMPTY"))); // line 46
    fmt.Println("-- snap-debug-info.sh: non-capturing group + \\s, -o --");
    os.WriteFile("grep_p_snap.txt", []byte("ID Status When\n1 Doing now\n2 Error now\n3 Doing now\n4 Error later\n"), 0644); // line 49
    DOINGS = s2s(capRun("'cat' 'grep_p_snap.txt' | 'tail' '-n' '+2' | 'grep' '-Eo' '([0-9]+[[:space:]]+Doing)' | 'awk' '{print $1}'", []string{"ADAPTER=" + s2s(ADAPTER), "ADAPTER3=" + s2s(ADAPTER3), "BGPID=" + s2s(BGPID), "CPID=" + s2s(CPID), "DEFAULT_ADAPTER=" + s2s(DEFAULT_ADAPTER), "DOINGS=" + s2s(DOINGS), "ERRORS=" + s2s(ERRORS), "N=" + s2s(N)})); vars["DOINGS"] = DOINGS;
    fmt.Println(fmt.Sprintf("doings=[%v]", DOINGS)); // line 51
    ERRORS = s2s(capRun("'cat' 'grep_p_snap.txt' | 'tail' '-n' '+2' | 'grep' '-Eo' '([0-9]+[[:space:]]+Error)' | 'awk' '{print $1}'", []string{"ADAPTER=" + s2s(ADAPTER), "ADAPTER3=" + s2s(ADAPTER3), "BGPID=" + s2s(BGPID), "CPID=" + s2s(CPID), "DEFAULT_ADAPTER=" + s2s(DEFAULT_ADAPTER), "DOINGS=" + s2s(DOINGS), "ERRORS=" + s2s(ERRORS), "N=" + s2s(N)})); vars["ERRORS"] = ERRORS;
    fmt.Println(fmt.Sprintf("errors=[%v]", ERRORS)); // line 53
    fmt.Println("-- softy: FQDN validation pattern (kept as grep -P) --");
    N = s2s(capRun("'echo' 'sub.example.com' | 'grep' '-P' '-c' '(?=^.{1,254}$)(^(?>(?!\\d+\\.)[a-zA-Z0-9_\\-]{1,63}\\.?)+(?:[a-zA-Z]{2,})$)'", []string{"ADAPTER=" + s2s(ADAPTER), "ADAPTER3=" + s2s(ADAPTER3), "BGPID=" + s2s(BGPID), "CPID=" + s2s(CPID), "DEFAULT_ADAPTER=" + s2s(DEFAULT_ADAPTER), "DOINGS=" + s2s(DOINGS), "ERRORS=" + s2s(ERRORS), "N=" + s2s(N)})); vars["N"] = N; // line 62
    fmt.Println(fmt.Sprintf("fqdn-count=%v", N)); // line 63
    N = s2s(capRun("'echo' '192.168.1.1' | 'grep' '-P' '-c' '(?=^.{1,254}$)(^(?>(?!\\d+\\.)[a-zA-Z0-9_\\-]{1,63}\\.?)+(?:[a-zA-Z]{2,})$)'", []string{"ADAPTER=" + s2s(ADAPTER), "ADAPTER3=" + s2s(ADAPTER3), "BGPID=" + s2s(BGPID), "CPID=" + s2s(CPID), "DEFAULT_ADAPTER=" + s2s(DEFAULT_ADAPTER), "DOINGS=" + s2s(DOINGS), "ERRORS=" + s2s(ERRORS), "N=" + s2s(N)})); vars["N"] = N; // line 64
    fmt.Println(fmt.Sprintf("ip-count=%v", N)); // line 65
    fmt.Println("-- memtester.sh / terminateProcess.sh: pgrep -P (untouched) --");
    BGPID = os.Getenv("!"); vars["BGPID"] = BGPID; // line 69
    CPID = s2s(capRun("( 'pgrep' '-P' \"$BGPID\" 'no-such-name-xyz' ) 2>/dev/null || 'true'", []string{"ADAPTER=" + s2s(ADAPTER), "ADAPTER3=" + s2s(ADAPTER3), "BGPID=" + s2s(BGPID), "CPID=" + s2s(CPID), "DEFAULT_ADAPTER=" + s2s(DEFAULT_ADAPTER), "DOINGS=" + s2s(DOINGS), "ERRORS=" + s2s(ERRORS), "N=" + s2s(N)})); vars["CPID"] = CPID; // line 70
    fmt.Println(fmt.Sprintf("pgrep-no-match=[%v]", pExp(":-", "CPID", "EMPTY"))); // line 71
    st = redirRun("( 'wait' \"$BGPID\" ) 2>/dev/null", []string{"ADAPTER=" + s2s(ADAPTER), "ADAPTER3=" + s2s(ADAPTER3), "BGPID=" + s2s(BGPID), "CPID=" + s2s(CPID), "DEFAULT_ADAPTER=" + s2s(DEFAULT_ADAPTER), "DOINGS=" + s2s(DOINGS), "ERRORS=" + s2s(ERRORS), "N=" + s2s(N)}); // line 72
    os.Exit(int(0)); // line 74
    _ = ADAPTER
    _ = ADAPTER3
    _ = BGPID
    _ = CPID
    _ = st
    return;
}