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

func b2i(b bool) int64 {
    if b { return 1 }
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
    var DEBUG any
    var __ps_tmp0 string
    var __ps_tmp1 string
    var __ps_tmp2 string
    var __ps_tmp3 string
    var __ps_tmp4 string
    var __ps_tmp5 string
    var basename string
    var config []any
    var config__host__ any
    var config__port__ any
    var config__user__ any
    var extension string
    var file any
    var filename any
    var first_half []any
    var i int64
    var input string
    var j any
    var last_half []any
    var message string
    var middle []any
    var name any
    var numbers []any
    var result string
    var sanitized string
    var uid any
    var uppercase string
    var st int64

    // TODO(unsupported): capture
    __ps_tmp1 = s2s(0); vars["__ps_tmp1"] = __ps_tmp1;
    // TODO(unsupported): Redirect
    // TODO(unsupported): capture
    __ps_tmp3 = s2s(0); vars["__ps_tmp3"] = __ps_tmp3;
    // TODO(unsupported): Redirect
    st = runCmd(exec.Command("diff", __ps_tmp1, __ps_tmp3));
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp1", None), Var("__ps_tmp3", None)], capture: None, redirects: [], env: [] }
    fmt.Println("Files:  file_a_1..10.txt file_a_1..10.log file_a_1..10.dat file_a_20.txt file_a_20.log file_a_20.dat file_a_30..40.txt file_a_30..40.log file_a_30..40.dat file_b_1..10.txt file_b_1..10.log file_b_1..10.dat file_b_20.txt file_b_20.log file_b_20.dat file_b_30..40.txt file_b_30..40.log file_b_30..40.dat file_c_1..10.txt file_c_1..10.log file_c_1..10.dat file_c_20.txt file_c_20.log file_c_20.dat file_c_30..40.txt file_c_30..40.log file_c_30..40.dat file_d_1..10.txt file_d_1..10.log file_d_1..10.dat file_d_20.txt file_d_20.log file_d_20.dat file_d_30..40.txt file_d_30..40.log file_d_30..40.dat file_e_1..10.txt file_e_1..10.log file_e_1..10.dat file_e_20.txt file_e_20.log file_e_20.dat file_e_30..40.txt file_e_30..40.log file_e_30..40.dat file_f_1..10.txt file_f_1..10.log file_f_1..10.dat file_f_20.txt file_f_20.log file_f_20.dat file_f_30..40.txt file_f_30..40.log file_f_30..40.dat file_g_1..10.txt file_g_1..10.log file_g_1..10.dat file_g_20.txt file_g_20.log file_g_20.dat file_g_30..40.txt file_g_30..40.log file_g_30..40.dat file_h_1..10.txt file_h_1..10.log file_h_1..10.dat file_h_20.txt file_h_20.log file_h_20.dat file_h_30..40.txt file_h_30..40.log file_h_30..40.dat file_i_1..10.txt file_i_1..10.log file_i_1..10.dat file_i_20.txt file_i_20.log file_i_20.dat file_i_30..40.txt file_i_30..40.log file_i_30..40.dat file_j_1..10.txt file_j_1..10.log file_j_1..10.dat file_j_20.txt file_j_20.log file_j_20.dat file_j_30..40.txt file_j_30..40.log file_j_30..40.dat file_k_1..10.txt file_k_1..10.log file_k_1..10.dat file_k_20.txt file_k_20.log file_k_20.dat file_k_30..40.txt file_k_30..40.log file_k_30..40.dat file_l_1..10.txt file_l_1..10.log file_l_1..10.dat file_l_20.txt file_l_20.log file_l_20.dat file_l_30..40.txt file_l_30..40.log file_l_30..40.dat file_m_1..10.txt file_m_1..10.log file_m_1..10.dat file_m_20.txt file_m_20.log file_m_20.dat file_m_30..40.txt file_m_30..40.log file_m_30..40.dat file_n_1..10.txt file_n_1..10.log file_n_1..10.dat file_n_20.txt file_n_20.log file_n_20.dat file_n_30..40.txt file_n_30..40.log file_n_30..40.dat file_o_1..10.txt file_o_1..10.log file_o_1..10.dat file_o_20.txt file_o_20.log file_o_20.dat file_o_30..40.txt file_o_30..40.log file_o_30..40.dat file_p_1..10.txt file_p_1..10.log file_p_1..10.dat file_p_20.txt file_p_20.log file_p_20.dat file_p_30..40.txt file_p_30..40.log file_p_30..40.dat file_q_1..10.txt file_q_1..10.log file_q_1..10.dat file_q_20.txt file_q_20.log file_q_20.dat file_q_30..40.txt file_q_30..40.log file_q_30..40.dat file_r_1..10.txt file_r_1..10.log file_r_1..10.dat file_r_20.txt file_r_20.log file_r_20.dat file_r_30..40.txt file_r_30..40.log file_r_30..40.dat file_s_1..10.txt file_s_1..10.log file_s_1..10.dat file_s_20.txt file_s_20.log file_s_20.dat file_s_30..40.txt file_s_30..40.log file_s_30..40.dat file_t_1..10.txt file_t_1..10.log file_t_1..10.dat file_t_20.txt file_t_20.log file_t_20.dat file_t_30..40.txt file_t_30..40.log file_t_30..40.dat file_u_1..10.txt file_u_1..10.log file_u_1..10.dat file_u_20.txt file_u_20.log file_u_20.dat file_u_30..40.txt file_u_30..40.log file_u_30..40.dat file_v_1..10.txt file_v_1..10.log file_v_1..10.dat file_v_20.txt file_v_20.log file_v_20.dat file_v_30..40.txt file_v_30..40.log file_v_30..40.dat file_w_1..10.txt file_w_1..10.log file_w_1..10.dat file_w_20.txt file_w_20.log file_w_20.dat file_w_30..40.txt file_w_30..40.log file_w_30..40.dat file_x_1..10.txt file_x_1..10.log file_x_1..10.dat file_x_20.txt file_x_20.log file_x_20.dat file_x_30..40.txt file_x_30..40.log file_x_30..40.dat file_y_1..10.txt file_y_1..10.log file_y_1..10.dat file_y_20.txt file_y_20.log file_y_20.dat file_y_30..40.txt file_y_30..40.log file_y_30..40.dat file_z_1..10.txt file_z_1..10.log file_z_1..10.dat file_z_20.txt file_z_20.log file_z_20.dat file_z_30..40.txt file_z_30..40.log file_z_30..40.dat");
    st = 0;
    name = "John Doe"; vars["name"] = name;
    st = 0;
    fmt.Println(fmt.Sprintf("Hello %v", "John_Doe"));
    st = 0;
    fmt.Println(fmt.Sprintf("Length: %v", "8"));
    st = 0;
    fmt.Println(fmt.Sprintf("First: %v", s2s("John")));
    st = 0;
    fmt.Println(fmt.Sprintf("Last: %v", strings.Join(pList("slice", "name", " -3", ""), " "))); // line 17
    st = 0;
    switch { // line 24
        case globMatch("[a-z]*", paramAt(1)):
            fmt.Println("Lowercase start");
            st = 0;
        case globMatch("[A-Z]*", paramAt(1)):
            fmt.Println("Uppercase start");
            st = 0;
        case globMatch("[0-9]*", paramAt(1)):
            fmt.Println("Number start");
            st = 0;
        case globMatch("?", paramAt(1)):
            fmt.Println("Single character");
            st = 0;
        case globMatch("*", paramAt(1)):
            fmt.Println("Something else");
            st = 0;
        default:
            _ = 0;
    }
    i = 2; vars["i"] = i;
    st = 0;
    t1 := i; // line 34
    i = i + 1; vars["i"] = i;
    i = i + 1;
    j = (t1 + i); vars["j"] = j;
    st = 0;
    fmt.Println(fmt.Sprintf("i=%v, j=%v", i, j)); // line 35
    st = 0;
    // declare config // line 38
    config__user__ = "admin"; vars["config[\"user\"]"] = config__user__;
    st = 0;
    config__host__ = "localhost"; vars["config[\"host\"]"] = config__host__;
    st = 0;
    config__port__ = "8080"; vars["config[\"port\"]"] = config__port__;
    st = 0;
    fmt.Print("User: $USER\nHost: ${HOSTNAME:-localhost}\nPath: $PWD\n"); // line 44
    st = 0;
    // TODO(unsupported): capture
    __ps_tmp4 = s2s(0); vars["__ps_tmp4"] = __ps_tmp4;
    st = 0;
    // TODO(unsupported): Redirect
    // TODO(unsupported): capture
    __ps_tmp5 = s2s(0); vars["__ps_tmp5"] = __ps_tmp5;
    st = 0;
    // TODO(unsupported): Redirect
    st = runPipe([][]string{[]string{"paste", __ps_tmp4, __ps_tmp5}, []string{"head", "-10"}});
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp4", None), Var("__ps_tmp5", None)], capture: None, redirects: [], env: [] }
    if (reMatch(s2s("^[0-9]+$"), s2s(paramAt(1))) && (strings.EqualFold(s2s(paramAt(2)), s2s("test")) || strings.EqualFold(s2s(paramAt(2)), s2s("debug")))) {
        fmt.Println("Valid input");
        st = 0;
    }
    st = runCmd(exec.Command("mkdir", "-p", "project/src/main/java", "project/src/main/resources", "project/src/test/java", "project/src/test/resources", "project/docs/api", "project/docs/user", "project/build/classes", "project/build/lib"));
    filename = "my_file.txt"; vars["filename"] = filename;
    st = 0;
    basename = s2s(pExp("%", "filename", ".*")); vars["basename"] = basename; // line 79
    st = 0;
    extension = s2s(pExp("##", "filename", "*.")); vars["extension"] = extension; // line 80
    st = 0;
    uppercase = "MY_FILE.TXT"; vars["uppercase"] = uppercase;
    st = 0;
    result = s2s(capRun("'echo' \"$(( $(wc -l < /etc/passwd) + $(wc -l < /etc/group) ))\"", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "__ps_tmp1=" + s2s(__ps_tmp1), "__ps_tmp2=" + s2s(__ps_tmp2), "__ps_tmp3=" + s2s(__ps_tmp3), "__ps_tmp4=" + s2s(__ps_tmp4), "__ps_tmp5=" + s2s(__ps_tmp5), "basename=" + s2s(basename), "extension=" + s2s(extension), "file=" + s2s(file), "filename=" + s2s(filename), "i=" + s2s(i), "input=" + s2s(input), "j=" + s2s(j), "message=" + s2s(message), "name=" + s2s(name), "result=" + s2s(result), "sanitized=" + s2s(sanitized), "uid=" + s2s(uid), "uppercase=" + s2s(uppercase)})); vars["result"] = result; // line 85
    st = 0;
    st = redirRun("'grep' '-v' '^#' '/etc/passwd' | 'cut' '-d' ':' '-f' '1,3' | 'sort' '-t' ':' '-k' '2' '-n' | ( 'head' '-5' ) >users.txt 2>errors.log", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "__ps_tmp1=" + s2s(__ps_tmp1), "__ps_tmp2=" + s2s(__ps_tmp2), "__ps_tmp3=" + s2s(__ps_tmp3), "__ps_tmp4=" + s2s(__ps_tmp4), "__ps_tmp5=" + s2s(__ps_tmp5), "basename=" + s2s(basename), "extension=" + s2s(extension), "file=" + s2s(file), "filename=" + s2s(filename), "i=" + s2s(i), "input=" + s2s(input), "j=" + s2s(j), "message=" + s2s(message), "name=" + s2s(name), "result=" + s2s(result), "sanitized=" + s2s(sanitized), "uid=" + s2s(uid), "uppercase=" + s2s(uppercase)});
    st = redirRun("( while 'read' '-r' 'user' 'pass' 'uid' 'gid' 'info' 'home' 'shell'; do if $(($(($uid)) > $((1000)))) && [ \"$shell\"!=\"/bin/false\" ]; then if [ \"$home\"=~^/home/ ]; then echo User: $user (UID: $uid) - $home; fi; fi; done ) 0</etc/passwd", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "__ps_tmp1=" + s2s(__ps_tmp1), "__ps_tmp2=" + s2s(__ps_tmp2), "__ps_tmp3=" + s2s(__ps_tmp3), "__ps_tmp4=" + s2s(__ps_tmp4), "__ps_tmp5=" + s2s(__ps_tmp5), "basename=" + s2s(basename), "extension=" + s2s(extension), "file=" + s2s(file), "filename=" + s2s(filename), "i=" + s2s(i), "input=" + s2s(input), "j=" + s2s(j), "message=" + s2s(message), "name=" + s2s(name), "result=" + s2s(result), "sanitized=" + s2s(sanitized), "uid=" + s2s(uid), "uppercase=" + s2s(uppercase)});
    numbers = []any{"1", "2", "3", "4", "5", "6", "7", "8", "9", "10"}; vars["numbers"] = numbers; // line 115
    middle = []any{strings.Join(pList("slice", "numbers", "3", "4"), " ")}; vars["middle"] = middle; // line 116
    first_half = []any{strings.Join(pList("slice", "numbers", "0", "5"), " ")}; vars["first_half"] = first_half; // line 117
    last_half = []any{strings.Join(pList("slice", "numbers", "5", ""), " ")}; vars["last_half"] = last_half; // line 118
    for _, file = range []any{"\x01SH2GLOB\x01*.txt", "\x01SH2GLOB\x01*.log", "\x01SH2GLOB\x01*.dat"} {
        vars["file"] = file;
        switch {
            case globMatch("*.txt", s2s(file)) || globMatch("*.log", s2s(file)):
                fmt.Println(fmt.Sprintf("Text file: %v", file));
                st = 0;
            case globMatch("*.dat", s2s(file)):
                fmt.Println(fmt.Sprintf("Data file: %v", file));
                st = 0;
            case globMatch("*", s2s(file)):
                fmt.Println(fmt.Sprintf("Other file: %v", file));
                st = 0;
            default:
                _ = 0;
        }
    }
    os.Setenv("DEBUG", "1"); // line 130
    os.Setenv("LOG_LEVEL", "verbose");
    DEBUG = nil;
    fmt.Println(fmt.Sprintf("Inner: LOG_LEVEL=%v, DEBUG=%v", os.Getenv("LOG_LEVEL"), pExp(":-", "DEBUG", "unset")));
    st = 0;
    fmt.Println(fmt.Sprintf("Outer: LOG_LEVEL=%v, DEBUG=%v", os.Getenv("LOG_LEVEL"), DEBUG));
    st = 0;
    message = fmt.Sprintf("Hello %v from %v", pExp(":-", "USER", "guest"), pExp(":-", "HOSTNAME", "localhost")); vars["message"] = message; // line 143
    st = 0;
    fmt.Println(fmt.Sprintf("%v", message)); // line 144
    st = 0;
    input = s2s(pExp(":-", "1", "default_value")); vars["input"] = input; // line 163
    st = 0;
    sanitized = s2s(pExp("//", "input", "[^a-zA-Z0-9]", "_")); vars["sanitized"] = sanitized; // line 164
    st = 0;
    uppercase = s2s(pExp("^^", "sanitized")); vars["uppercase"] = uppercase; // line 165
    st = 0;
    fmt.Println(fmt.Sprintf("Input: '%v' -> Sanitized: '%v' -> Uppercase: '%v'", input, sanitized, uppercase)); // line 166
    st = 0;
    st = b2i(!(fileTest("f", s2s(paramAt(1)))));
    if st == 0 {
        st = runCmd(exec.Command("echo", "File exists"));
    }
    if st != 0 {
        st = runCmd(exec.Command("echo", "File not found"));
    }
    st = b2i(!(fileTest("d", s2s(paramAt(2)))));
    if st == 0 {
        st = runCmd(exec.Command("cd", paramAt(2)));
    }
    if st == 0 {
        st = runCmd(exec.Command("pwd"));
    }
    if st != 0 {
        st = runCmd(exec.Command("echo", "Directory not accessible"));
    }
    fmt.Println("Script completed successfully!");
    st = 0;
    _ = __ps_tmp0
    _ = __ps_tmp2
    _ = basename
    _ = config
    _ = config__host__
    _ = config__port__
    _ = config__user__
    _ = extension
    _ = filename
    _ = first_half
    _ = last_half
    _ = middle
    _ = name
    _ = numbers
    _ = result
    _ = uid
    _ = st
    return;
}
// 10 construct(s) lowered to TODO markers