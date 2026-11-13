package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "regexp"
    "strings"
)


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

// textLines is the LINES of a file's text: the trailing newline ends the
// last line instead of opening an empty one, and an empty file has none.
func textLines(data []byte) []string {
    if len(data) == 0 { return nil }
    return strings.Split(strings.TrimSuffix(string(data), "\n"), "\n")
}

// commFiles is GNU comm's merge over two sorted files: the lines only in
// the first (column 1), only in the second (column 2) and in both
// (column 3). A suppressed column is not written, and the remaining
// columns keep GNU's rule that each one is indented by ONE TAB PER ENABLED
// column before it (`-12` therefore prints column 3 with no indent).
// Out-of-order input is what GNU warns about and still prints, with
// status 1; stderr is not this helper's to write.
func commFiles(a, b string, c1, c2, c3 bool) (string, int64) {
    da, ea := os.ReadFile(a)
    db, eb := os.ReadFile(b)
    if ea != nil || eb != nil { return "", 1 }
    pad := func(col int) string {
        n := 0
        if c1 && col > 1 { n++ }
        if c2 && col > 2 { n++ }
        return strings.Repeat("\t", n)
    }
    la, lb := textLines(da), textLines(db)
    var out strings.Builder
    var status int64
    // prev* are the last line each file contributed, so a decrease
    // between them is the out-of-order case.
    var pa, pb string
    i, j := 0, 0
    for i < len(la) || j < len(lb) {
        switch {
        case i >= len(la):
            if pb > lb[j] { status = 1 }
            if c2 { out.WriteString(pad(2)); out.WriteString(lb[j]); out.WriteByte('\n') }
            pb = lb[j]; j++
        case j >= len(lb):
            if pa > la[i] { status = 1 }
            if c1 { out.WriteString(la[i]); out.WriteByte('\n') }
            pa = la[i]; i++
        case la[i] == lb[j]:
            if pa > la[i] || pb > lb[j] { status = 1 }
            if c3 { out.WriteString(pad(3)); out.WriteString(la[i]); out.WriteByte('\n') }
            pa, pb = la[i], lb[j]; i++; j++
        case la[i] < lb[j]:
            if pa > la[i] { status = 1 }
            if c1 { out.WriteString(la[i]); out.WriteByte('\n') }
            pa = la[i]; i++
        default:
            if pb > lb[j] { status = 1 }
            if c2 { out.WriteString(pad(2)); out.WriteString(lb[j]); out.WriteByte('\n') }
            pb = lb[j]; j++
        }
    }
    return out.String(), status
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

func eEsc(s string) string {
    var out strings.Builder
    for i := 0; i < len(s); i++ {
        c := s[i]
        if c != '\\' || i+1 >= len(s) { out.WriteByte(c); continue }
        i++
        switch s[i] {
        case 'n': out.WriteByte('\n')
        case 't': out.WriteByte('\t')
        case 'r': out.WriteByte('\r')
        case 'a': out.WriteByte('\a')
        case 'b': out.WriteByte('\b')
        case 'f': out.WriteByte('\f')
        case 'v': out.WriteByte('\v')
        case '\\': out.WriteByte('\\')
        case '0', '1', '2', '3', '4', '5', '6', '7': {
            v := int(s[i] - '0')
            for k := 0; k < 2 && i+1 < len(s) && s[i+1] >= '0' && s[i+1] <= '7'; k++ {
                i++; v = v*8 + int(s[i]-'0')
            }
            out.WriteByte(byte(v))
        }
        default: out.WriteByte('\\'); out.WriteByte(s[i])
        }
    }
    return out.String()
}
func main() {
    var awk_result string
    var comm_result string
    var cut_result string
    var diff_result string
    var file_content string
    var grep_result string
    var head_result string
    var line_count string
    var paste_result string
    var sed_result string
    var sort_result string
    var tail_result string
    var tr_result string
    var uniq_result string
    var word_count string
    var xargs_result string
    var st int64

    fmt.Println("=== Text Processing Commands ===");
    st = 0;
    file_content = capPipe([][]string{[]string{"cat", "src/main.rs"}, []string{"head", "-5"}}); // line 10
    st = 0;
    fmt.Println("First 5 lines of main.rs:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", file_content)); // line 12
    st = 0;
    grep_result = func() string { __gre, __gce := regexp.Compile("fn"); if __gce != nil { st = 2; return "" }; __gd, __gde := os.ReadFile("src/main.rs"); if __gde != nil { st = 2; return "" }; __gls := strings.Split(string(__gd), "\n"); if len(__gls) > 0 && __gls[len(__gls)-1] == "" { __gls = __gls[:len(__gls)-1] }; __gc := 0; __go := ""; for __gi, __gl := range __gls { __gm := __gre.MatchString(__gl); if __gm { __gc++; __go += fmt.Sprintf("%d:%s\n", __gi+1, __gl) } }; if __gc > 0 { st = 0 } else { st = 1 }; return strings.TrimRight(__go, "\n") }();
    fmt.Println("Lines containing 'fn':");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", grep_result)); // line 18
    st = 0;
    sed_result = capPipe([][]string{[]string{"echo", "Hello World"}, []string{"sed", "s/World/Universe/"}});
    st = 0;
    fmt.Println(fmt.Sprintf("Sed result: %v", sed_result)); // line 23
    st = 0;
    awk_result = capPipe([][]string{[]string{"echo", "1 2 3 4 5"}, []string{"awk", "{print $1 + $2}"}});
    st = 0;
    fmt.Println(fmt.Sprintf("Awk sum result: %v", awk_result)); // line 28
    st = 0;
    sort_result = capPipe([][]string{[]string{"echo", "-e", "zebra\\napple\\nbanana"}, []string{"sort"}});
    st = 0;
    fmt.Println("Sorted words:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", sort_result)); // line 34
    st = 0;
    uniq_result = capPipe([][]string{[]string{"echo", "-e", "apple\\napple\\nbanana\\nbanana\\ncherry"}, []string{"uniq"}});
    st = 0;
    fmt.Println("Unique words:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", uniq_result)); // line 40
    st = 0;
    word_count = capPipe([][]string{[]string{"echo", "Hello World"}, []string{"wc", "-w"}});
    st = 0;
    line_count = capPipe([][]string{[]string{"echo", "-e", "line1\\nline2\\nline3"}, []string{"wc", "-l"}});
    st = 0;
    fmt.Println(fmt.Sprintf("Word count: %v", word_count)); // line 46
    st = 0;
    fmt.Println(fmt.Sprintf("Line count: %v", line_count)); // line 47
    st = 0;
    head_result = capPipe([][]string{[]string{"seq", "1", "10"}, []string{"head", "-3"}}); // line 51
    st = 0;
    fmt.Println(fmt.Sprintf("First 3 numbers: %v", head_result)); // line 52
    st = 0;
    tail_result = capPipe([][]string{[]string{"seq", "1", "10"}, []string{"tail", "-3"}}); // line 56
    st = 0;
    fmt.Println(fmt.Sprintf("Last 3 numbers: %v", tail_result)); // line 57
    st = 0;
    cut_result = capPipe([][]string{[]string{"echo", "apple:banana:cherry"}, []string{"cut", "-d", ":", "-f", "2"}});
    st = 0;
    fmt.Println(fmt.Sprintf("Second field: %v", cut_result)); // line 62
    st = 0;
    if __wet1 := os.WriteFile("temp1.txt", []byte(fmt.Sprintf("%v\n", eEsc("1\\n2\\n3"))), 0644); __wet1 != nil { st = 1 } else { st = 0 }
    if __wet2 := os.WriteFile("temp2.txt", []byte(fmt.Sprintf("%v\n", eEsc("a\\nb\\nc"))), 0644); __wet2 != nil { st = 1 } else { st = 0 }
    paste_result = capPipe([][]string{[]string{"paste", "temp1.txt", "temp2.txt"}, []string{"sed", "s/\\t/ /g"}}); // line 68
    st = 0;
    fmt.Println("Pasted columns:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", paste_result)); // line 70
    st = 0;
    st = 0; // line 71
    if err := os.Remove("temp1.txt"); err != nil && !os.IsNotExist(err) { st = 1 }
    if err := os.Remove("temp2.txt"); err != nil && !os.IsNotExist(err) { st = 1 }
    if __wet4 := os.WriteFile("file1.txt", []byte(fmt.Sprintf("%v\n", eEsc("apple\\nbanana\\ncherry"))), 0644); __wet4 != nil { st = 1 } else { st = 0 }
    if __wet5 := os.WriteFile("file2.txt", []byte(fmt.Sprintf("%v\n", eEsc("banana\\ncherry\\ndate"))), 0644); __wet5 != nil { st = 1 } else { st = 0 }
    comm_result = func() string { __coct6, _ := commFiles("file1.txt", "file2.txt", false, false, true); return strings.TrimRight(__coct6, "\n") }(); // line 77
    st = 0;
    fmt.Println("Common lines:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", comm_result)); // line 79
    st = 0;
    diff_result = func() string { __cc7 := exec.Command("diff", "file1.txt", "file2.txt"); shStdIn(__cc7); shStdErr(__cc7); __co7, _ := __cc7.Output(); return strings.TrimRight(string(__co7), "\n") }(); // line 83
    st = 0;
    fmt.Println("File differences:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", diff_result)); // line 85
    st = 0;
    tr_result = capPipe([][]string{[]string{"echo", "HELLO WORLD"}, []string{"tr", "A-Z", "a-z"}});
    st = 0;
    fmt.Println(fmt.Sprintf("Lowercase: %v", tr_result)); // line 90
    st = 0;
    xargs_result = capPipe([][]string{[]string{"echo", "1 2 3"}, []string{"xargs", "-n", "1", "echo", "Number:"}});
    st = 0;
    fmt.Println("Xargs result:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", xargs_result)); // line 96
    st = 0;
    st = 0; // line 99
    if err := os.Remove("file1.txt"); err != nil && !os.IsNotExist(err) { st = 1 }
    if err := os.Remove("file2.txt"); err != nil && !os.IsNotExist(err) { st = 1 }
    os.Exit(int(st));
}