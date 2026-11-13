package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "regexp"
    "strconv"
    "strings"
)


// pipeStatus is bash's PIPESTATUS: the exit code of EVERY stage of the
// last pipeline (not just the last one).
var pipeStatus []int64

func exitCode(err error) int64 {
    if err == nil { return 0 }
    if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
    return 1
}

func runPipe(stages [][]string) int64 {
    var cmds []*exec.Cmd
    for _, st := range stages {
        c := exec.Command(st[0], st[1:]...)
        cmds = append(cmds, c)
    }
    if len(cmds) == 0 { return 0 }
    var pipes []io.Closer
    for i := 0; i < len(cmds)-1; i++ {
        p, err := cmds[i].StdoutPipe()
        if err != nil { return 1 }
        cmds[i+1].Stdin = p
        pipes = append(pipes, p)
    }
    cmds[len(cmds)-1].Stdout = os.Stdout
    // After the pipe wiring (StdoutPipe refuses an already-set Stdout):
    // unwired Stdin ends follow the current process stdin.
    for _, c := range cmds { if c.Stdin == nil { c.Stdin = os.Stdin } }
    codes := make([]int64, len(cmds))
    last := int64(0)
    for i, c := range cmds {
        if i < len(cmds)-1 {
            if err := c.Start(); err != nil { codes[i] = 1 }
        } else {
            last = exitCode(c.Run())
            codes[i] = last
        }
    }
    // The parent holds a read end of every inter-stage pipe: close
    // the parent copies after Run so an early-exiting downstream
    // stage (head) retires the producers instead of deadlocking them.
    for _, p := range pipes { p.Close() }
    for i := 0; i < len(cmds)-1; i++ {
        codes[i] = exitCode(cmds[i].Wait())
    }
    pipeStatus = codes
    return last
}

func capPipe(stages [][]string) string {
    return strings.TrimRight(capPipeRaw(stages), "\n")
}

// The capture form of runPipe: same native stage chain, but the LAST
// stage's stdout is read instead of inherited, so `$(cmd | cmd ...)` needs
// no `bash -c` child. Both the read and the Waits must happen before the
// last stage is reaped - a StdoutPipe is not valid after Wait.
func capPipeRaw(stages [][]string) string {
    var cmds []*exec.Cmd
    for _, st := range stages {
        if len(st) == 0 { return "" }
        c := exec.Command(st[0], st[1:]...)
        cmds = append(cmds, c)
    }
    if len(cmds) == 0 { return "" }
    var pipes []io.Closer
    for i := 0; i < len(cmds)-1; i++ {
        p, err := cmds[i].StdoutPipe()
        if err != nil { return "" }
        cmds[i+1].Stdin = p
        pipes = append(pipes, p)
    }
    last, err := cmds[len(cmds)-1].StdoutPipe()
    if err != nil { return "" }
    // After the pipe wiring: unwired Stdin ends follow the process stdin.
    for _, c := range cmds { if c.Stdin == nil { c.Stdin = os.Stdin } }
    for _, c := range cmds {
        if err := c.Start(); err != nil {
            for _, d := range cmds { if d.Process != nil { d.Process.Kill() } }
            for _, d := range cmds { d.Wait() }
            return ""
        }
    }
    data, err := io.ReadAll(last)
    if err != nil { data = nil }
    // Same parent-holds-the-pipe deadlock as runPipe (see above).
    for _, p := range pipes { p.Close() }
    for _, c := range cmds { c.Wait() }
    return string(data)
}

var origStderr = os.Stderr

// shStd wires a child to the CURRENT process stdio: a nil Stdin
    // follows the (possibly swapped) os.Stdin var, and a nil Stderr
    // follows a swapped os.Stderr but stays discarded otherwise
    // (native helpers never write stderr by default). Stdout is left
    // alone: capturers attach their own pipe (pre-setting it breaks
    // Output), and runners default it explicitly. Function bodies run
    // with these vars temporarily swapped for welded redirects, so
    // every spawn site funnels through here instead of inheriting the
    // raw fds (which the swap does not change).
func shStdIn(c *exec.Cmd) {
    if c.Stdin == nil { c.Stdin = os.Stdin }
}

func shStdErr(c *exec.Cmd) {
    if c.Stderr == nil {
        if os.Stderr != origStderr { c.Stderr = os.Stderr } else { c.Stderr = io.Discard }
    }
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
            case 'q': out.WriteString(bShQuote(arg))
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

// bShQuote is `printf %q`: the word rewritten so the shell reads it
// back unchanged. bash leaves printable safe characters bare, escapes
// every other printable with a backslash (`a b` -> `a\ b`), and uses
// the ANSI-C form when the text carries control characters
// (`$'\n\t'`) — `strconv.Quote` produces the last case's Go spelling,
// which is not a shell word.
func bShQuote(s string) string {
    if s == "" { return "''" }
    ansi := false
    for i := 0; i < len(s); i++ {
        if c := s[i]; c < 0x20 || c == 0x7f { ansi = true; break }
    }
    var b strings.Builder
    if ansi {
        b.WriteString("$'")
        for i := 0; i < len(s); i++ {
            c := s[i]
            switch c {
            case 7: b.WriteString("\\a")
            case 8: b.WriteString("\\b")
            case 12: b.WriteString("\\f")
            case '\n': b.WriteString("\\n")
            case '\r': b.WriteString("\\r")
            case '\t': b.WriteString("\\t")
            case 11: b.WriteString("\\v")
            case '\\': b.WriteString("\\\\")
            case '\'': b.WriteString("\\'")
            default:
                if c < 0x20 || c == 0x7f {
                    d := strconv.FormatInt(int64(c), 8)
                    b.WriteByte('\\')
                    for len(d) < 3 { d = "0" + d }
                    b.WriteString(d)
                } else {
                    b.WriteByte(c)
                }
            }
        }
        b.WriteString("'")
        return b.String()
    }
    const safe = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_@%+=:,./-"
    for i := 0; i < len(s); i++ {
        if strings.IndexByte(safe, s[i]) < 0 { b.WriteByte('\\') }
        b.WriteByte(s[i])
    }
    return b.String()
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
    var count string
    var found string
    var lineno string
    var longline string
    var matched string
    var st int64

    fmt.Println("== Basic grep parameters ==");
    st = 0;
    st = runPipe([][]string{[]string{"echo", "text with pattern"}, []string{"grep", "-i", "PATTERN"}});
    if st == 0 {
        fmt.Println("  -i match: OK");
        st = 0;
    }
    if st != 0 {
        fmt.Println("  -i match: FAIL");
        st = 0;
    }
    count = capPipe([][]string{[]string{"echo", "-e", "line1\\nline2\\nline3"}, []string{"grep", "-v", "line2"}, []string{"wc", "-l"}});
    st = 0;
    fmt.Println(fmt.Sprintf("  -v count: %v (expected 2)", count)); // line 10
    st = 0;
    matched = capPipe([][]string{[]string{"echo", "-e", "match\\nno match\\nmatch again"}, []string{"grep", "-c", "match"}});
    st = 0;
    fmt.Println(fmt.Sprintf("  -c count: %v (expected 2)", matched)); // line 13
    st = 0;
    fmt.Println("== Context parameters ==");
    st = 0;
    st = func() int64 {
var __pqvt1 int64;
__pqokt2 := true;
__pqstt2 := int64(0);
__pqt2_0 := exec.Command("echo", "-e", "line1\\nline2\\nTARGET\\nline4\\nline5");
__pqt2_1 := exec.Command("grep", "-A", "2", "TARGET");
__pqt2_p0, __pqt2_pe0 := __pqt2_0.StdoutPipe();
if __pqt2_pe0 != nil { __pqokt2 = false } else { __pqt2_1.Stdin = __pqt2_p0 }
__pqt2_f0, __pqt2_fe0 := os.OpenFile("/tmp/grep_out.txt", os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0644);
if __pqt2_fe0 == nil { __pqt2_1.Stdout = __pqt2_f0 } else { __pqokt2 = false }
__pqt2_0.Stderr = io.Discard;
__pqt2_1.Stderr = io.Discard;
if __pqokt2 {
__pqstt2 = func() int64 {
if __pqt2_0.Stdin == nil { __pqt2_0.Stdin = os.Stdin }
if __e := __pqt2_0.Start(); __e != nil { return 1 }
if __e := __pqt2_1.Run(); __e != nil { if __ee, __ok := __e.(*exec.ExitError); __ok { return int64(__ee.ExitCode()) }; return 1 }
__pqt2_p0.Close();
__pqt2_0.Wait();
return 0;
}();
} else {
__pqstt2 = 1;
}
if __pqt2_fe0 == nil { __pqt2_f0.Close() }
__pqvt1 = __pqstt2;
return __pqvt1;
}();
    fmt.Println(fmt.Sprintf("  -A 2 lines: %v (expected 3)", func() string { __wdt3, __wdet3 := os.ReadFile("/tmp/grep_out.txt"); if __wdet3 != nil { return "" }; return strconv.FormatInt(int64(strings.Count(string(__wdt3), "\n")), 10) }())); // line 17
    st = 0;
    st = func() int64 {
var __pqvt4 int64;
__pqokt5 := true;
__pqstt5 := int64(0);
__pqt5_0 := exec.Command("echo", "-e", "line1\\nline2\\nTARGET\\nline4\\nline5");
__pqt5_1 := exec.Command("grep", "-B", "2", "TARGET");
__pqt5_p0, __pqt5_pe0 := __pqt5_0.StdoutPipe();
if __pqt5_pe0 != nil { __pqokt5 = false } else { __pqt5_1.Stdin = __pqt5_p0 }
__pqt5_f0, __pqt5_fe0 := os.OpenFile("/tmp/grep_out.txt", os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0644);
if __pqt5_fe0 == nil { __pqt5_1.Stdout = __pqt5_f0 } else { __pqokt5 = false }
__pqt5_0.Stderr = io.Discard;
__pqt5_1.Stderr = io.Discard;
if __pqokt5 {
__pqstt5 = func() int64 {
if __pqt5_0.Stdin == nil { __pqt5_0.Stdin = os.Stdin }
if __e := __pqt5_0.Start(); __e != nil { return 1 }
if __e := __pqt5_1.Run(); __e != nil { if __ee, __ok := __e.(*exec.ExitError); __ok { return int64(__ee.ExitCode()) }; return 1 }
__pqt5_p0.Close();
__pqt5_0.Wait();
return 0;
}();
} else {
__pqstt5 = 1;
}
if __pqt5_fe0 == nil { __pqt5_f0.Close() }
__pqvt4 = __pqstt5;
return __pqvt4;
}();
    fmt.Println(fmt.Sprintf("  -B 2 lines: %v (expected 3)", func() string { __wdt6, __wdet6 := os.ReadFile("/tmp/grep_out.txt"); if __wdet6 != nil { return "" }; return strconv.FormatInt(int64(strings.Count(string(__wdt6), "\n")), 10) }())); // line 20
    st = 0;
    st = func() int64 {
var __pqvt7 int64;
__pqokt8 := true;
__pqstt8 := int64(0);
__pqt8_0 := exec.Command("echo", "-e", "line1\\nline2\\nTARGET\\nline4\\nline5");
__pqt8_1 := exec.Command("grep", "-C", "1", "TARGET");
__pqt8_p0, __pqt8_pe0 := __pqt8_0.StdoutPipe();
if __pqt8_pe0 != nil { __pqokt8 = false } else { __pqt8_1.Stdin = __pqt8_p0 }
__pqt8_f0, __pqt8_fe0 := os.OpenFile("/tmp/grep_out.txt", os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0644);
if __pqt8_fe0 == nil { __pqt8_1.Stdout = __pqt8_f0 } else { __pqokt8 = false }
__pqt8_0.Stderr = io.Discard;
__pqt8_1.Stderr = io.Discard;
if __pqokt8 {
__pqstt8 = func() int64 {
if __pqt8_0.Stdin == nil { __pqt8_0.Stdin = os.Stdin }
if __e := __pqt8_0.Start(); __e != nil { return 1 }
if __e := __pqt8_1.Run(); __e != nil { if __ee, __ok := __e.(*exec.ExitError); __ok { return int64(__ee.ExitCode()) }; return 1 }
__pqt8_p0.Close();
__pqt8_0.Wait();
return 0;
}();
} else {
__pqstt8 = 1;
}
if __pqt8_fe0 == nil { __pqt8_f0.Close() }
__pqvt7 = __pqstt8;
return __pqvt7;
}();
    fmt.Println(fmt.Sprintf("  -C 1 lines: %v (expected 3)", func() string { __wdt9, __wdet9 := os.ReadFile("/tmp/grep_out.txt"); if __wdet9 != nil { return "" }; return strconv.FormatInt(int64(strings.Count(string(__wdt9), "\n")), 10) }())); // line 23
    st = 0;
    fmt.Println("== File handling parameters ==");
    st = 0;
    if __wet10 := os.WriteFile("/tmp/grep_file.txt", []byte("content\n"), 0644); __wet10 != nil { st = 1 } else { st = 0 }
    st = func() int64 { __gre, __gce := regexp.Compile("content"); if __gce != nil { return 2 }; __gd, __gde := os.ReadFile("/tmp/grep_file.txt"); if __gde != nil { return 2 }; __gls := strings.Split(string(__gd), "\n"); if len(__gls) > 0 && __gls[len(__gls)-1] == "" { __gls = __gls[:len(__gls)-1] }; __gc := 0; __go := ""; for _, __gl := range __gls { __gm := __gre.MatchString(__gl); if __gm { __gc++ } }; __go = fmt.Sprintf("%d\n", __gc); fmt.Print(__go); if __gc > 0 { return 0 }; return 1 }();
    if st == 0 {
        fmt.Println("  -c file: OK");
        st = 0;
    }
    if st != 0 {
        fmt.Println("  -c file: FAIL");
        st = 0;
    }
    st = func() int64 { __gre, __gce := regexp.Compile("content"); if __gce != nil { return 2 }; __gd, __gde := os.ReadFile("/tmp/grep_file.txt"); if __gde != nil { return 2 }; __gls := strings.Split(string(__gd), "\n"); if len(__gls) > 0 && __gls[len(__gls)-1] == "" { __gls = __gls[:len(__gls)-1] }; __gc := 0; __go := ""; for _, __gl := range __gls { __gm := __gre.MatchString(__gl); if __gm { __gc++ } }; if __gc > 0 { __go = "/tmp/grep_file.txt" + "\n" }; fmt.Print(__go); if __gc > 0 { return 0 }; return 1 }();
    if st == 0 {
        fmt.Println("  -l: found");
        st = 0;
    }
    if st != 0 {
        fmt.Println("  -l: not found");
        st = 0;
    }
    st = func() int64 { __ec11 := exec.Command("grep", "-L", "nonexistent", "/tmp/grep_file.txt"); shStdIn(__ec11); shStdErr(__ec11); __ec11.Stdout = os.Stdout;; st = 0; if __ee11 := __ec11.Run(); __ee11 != nil { st = 127; if __ec11.ProcessState != nil { st = int64(__ec11.ProcessState.ExitCode()) } }; return st }();
    if st == 0 {
        fmt.Println("  -L: not found (correct)");
        st = 0;
    }
    if st != 0 {
        fmt.Println("  -L: found (wrong)");
        st = 0;
    }
    fmt.Println("== Output formatting parameters ==");
    st = 0;
    matched = strings.Join(regexp.MustCompile("pattern").FindAllString("text with pattern in it", -1), "\n");
    st = 0;
    fmt.Println(fmt.Sprintf("  -o match: '%v' (expected 'pattern')", matched)); // line 33
    st = 0;
    lineno = capPipe([][]string{[]string{"echo", "text with pattern in it"}, []string{"grep", "-n", "pattern"}, []string{"cut", "-d", ":", "-f", "1"}});
    st = 0;
    fmt.Println(fmt.Sprintf("  -n line: %v (expected 1)", lineno)); // line 36
    st = 0;
    fmt.Println("== Recursive parameters ==");
    st = 0;
    st = 0;
    if err := os.MkdirAll("/tmp/grep_sub", 0o777); err != nil { st = 1 }
    if st == 0 {
        st = func() int64 { __grkt12 := true; __grot12_1 := os.Stdout; __grft12_0, __gret12_0 := os.OpenFile("/tmp/grep_sub/file.txt", os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0644); if __gret12_0 == nil { os.Stdout = __grft12_0 } else { __grkt12 = false }; if __grkt12 {;     fmt.Println("subfile content");;     st = 0;; } else {; st = 1; }; os.Stdout = __grot12_1; if __gret12_0 == nil { __grft12_0.Close() }; return st }();
    }
    found = capPipe([][]string{[]string{"grep", "-r", "subfile", "/tmp/grep_sub"}, []string{"wc", "-l"}});
    st = 0;
    fmt.Println(fmt.Sprintf("  -r recursive: %v files matched (expected 1)", found)); // line 41
    st = 0;
    st = 0; // line 42
    if err := os.RemoveAll("/tmp/grep_sub"); err != nil { st = 1 }
    fmt.Println("== Line length parameters ==");
    st = 0;
    longline = strings.TrimRight(bprintfStr("a%.0s", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "100", "101", "102", "103", "104", "105", "106", "107", "108", "109", "110", "111", "112", "113", "114", "115", "116", "117", "118", "119", "120", "121", "122", "123", "124", "125", "126", "127", "128", "129", "130", "131", "132", "133", "134", "135", "136", "137", "138", "139", "140", "141", "142", "143", "144", "145", "146", "147", "148", "149", "150", "151", "152", "153", "154", "155", "156", "157", "158", "159", "160", "161", "162", "163", "164", "165", "166", "167", "168", "169", "170", "171", "172", "173", "174", "175", "176", "177", "178", "179", "180", "181", "182", "183", "184", "185", "186", "187", "188", "189", "190", "191", "192", "193", "194", "195", "196", "197", "198", "199", "200"), "\n");
    st = 0;
    st = func() int64 {
var __pqvt14 int64;
__pqokt15 := true;
__pqstt15 := int64(0);
__pqt15_0 := exec.Command("echo", longline);
__pqt15_1 := exec.Command("grep", "-m", "1", "a");
__pqt15_p0, __pqt15_pe0 := __pqt15_0.StdoutPipe();
if __pqt15_pe0 != nil { __pqokt15 = false } else { __pqt15_1.Stdin = __pqt15_p0 }
__pqt15_f0, __pqt15_fe0 := os.OpenFile("/dev/null", os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0644);
if __pqt15_fe0 == nil { __pqt15_1.Stdout = __pqt15_f0 } else { __pqokt15 = false }
__pqt15_0.Stderr = io.Discard;
__pqt15_1.Stderr = io.Discard;
if __pqokt15 {
__pqstt15 = func() int64 {
if __pqt15_0.Stdin == nil { __pqt15_0.Stdin = os.Stdin }
if __e := __pqt15_0.Start(); __e != nil { return 1 }
if __e := __pqt15_1.Run(); __e != nil { if __ee, __ok := __e.(*exec.ExitError); __ok { return int64(__ee.ExitCode()) }; return 1 }
__pqt15_p0.Close();
__pqt15_0.Wait();
return 0;
}();
} else {
__pqstt15 = 1;
}
if __pqt15_fe0 == nil { __pqt15_f0.Close() }
__pqvt14 = __pqstt15;
return __pqvt14;
}();
    if st == 0 {
        fmt.Println("  -m 1 (long line): OK");
        st = 0;
    }
    if st != 0 {
        fmt.Println("  -m 1 (long line): FAIL");
        st = 0;
    }
    fmt.Println("== Word-regexp and line-regexp parameters ==");
    st = 0;
    st = func() int64 {
var __pqvt16 int64;
__pqokt17 := true;
__pqstt17 := int64(0);
__pqt17_0 := exec.Command("echo", "-e", "foo\\nfoobar\\nbar");
__pqt17_1 := exec.Command("grep", "-w", "foo");
__pqt17_p0, __pqt17_pe0 := __pqt17_0.StdoutPipe();
if __pqt17_pe0 != nil { __pqokt17 = false } else { __pqt17_1.Stdin = __pqt17_p0 }
__pqt17_f0, __pqt17_fe0 := os.OpenFile("/tmp/grep_out.txt", os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0644);
if __pqt17_fe0 == nil { __pqt17_1.Stdout = __pqt17_f0 } else { __pqokt17 = false }
__pqt17_0.Stderr = io.Discard;
__pqt17_1.Stderr = io.Discard;
if __pqokt17 {
__pqstt17 = func() int64 {
if __pqt17_0.Stdin == nil { __pqt17_0.Stdin = os.Stdin }
if __e := __pqt17_0.Start(); __e != nil { return 1 }
if __e := __pqt17_1.Run(); __e != nil { if __ee, __ok := __e.(*exec.ExitError); __ok { return int64(__ee.ExitCode()) }; return 1 }
__pqt17_p0.Close();
__pqt17_0.Wait();
return 0;
}();
} else {
__pqstt17 = 1;
}
if __pqt17_fe0 == nil { __pqt17_f0.Close() }
__pqvt16 = __pqstt17;
return __pqvt16;
}();
    fmt.Println(fmt.Sprintf("  -w word match lines: %v (expected 1)", func() string { __wdt18, __wdet18 := os.ReadFile("/tmp/grep_out.txt"); if __wdet18 != nil { return "" }; return strconv.FormatInt(int64(strings.Count(string(__wdt18), "\n")), 10) }())); // line 50
    st = 0;
    st = 0; // line 52
    if err := os.Remove("/tmp/grep_file.txt"); err != nil && !os.IsNotExist(err) { st = 1 }
    if err := os.Remove("/tmp/grep_out.txt"); err != nil && !os.IsNotExist(err) { st = 1 }
    _ = st
    os.Exit(int(st));
}