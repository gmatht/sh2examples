package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
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
    var d string
    var st int64

    d = func() string { __mknt1, __mket1 := os.MkdirTemp("", "tmp*"); if __mket1 != nil { return "" }; return __mknt1 }(); // line 4
    st = 0;
    if err := os.Chdir(d); err != nil { st = 1 } else { st = 0 }; // line 5
    if st != 0 {
        os.Exit(int(1));
    }
    if __wet2 := os.WriteFile("file.txt", []byte(bprintfStr("apple\\napple\\n")), 0644); __wet2 != nil { st = 1 } else { st = 0 } // line 6
    if __wet3 := os.WriteFile("note.txt", []byte(bprintfStr("hello.txt\\n")), 0644); __wet3 != nil { st = 1 } else { st = 0 } // line 7
    if __wet4 := os.WriteFile("s.sh", []byte(bprintfStr("function f() { echo hi; }\\n")), 0644); __wet4 != nil { st = 1 } else { st = 0 } // line 8
    st = runPipe([][]string{[]string{"ls"}, []string{"grep", "\\.txt$"}, []string{"wc", "-l"}});
    fmt.Println(""); // line 10
    st = 0;
    st = runPipe([][]string{[]string{"cat", "file.txt"}, []string{"sort"}, []string{"uniq", "-c"}, []string{"sort", "-n", "-r"}}); // line 11
    fmt.Println(""); // line 12
    st = 0;
    st = runPipe([][]string{[]string{"find", ".", "-name", "*.sh"}, []string{"xargs", "grep", "-l", "function"}, []string{"tr", "-d", "\\\\/"}});
    fmt.Println(""); // line 14
    st = 0;
    st = runPipe([][]string{[]string{"cat", "file.txt"}, []string{"tr", "a", "b"}, []string{"grep", "hello"}});
    fmt.Println(""); // line 16
    st = 0;
    st = runPipe([][]string{[]string{"cat", "file.txt"}, []string{"sort"}, []string{"grep", "hello"}}); // line 17
    if err := os.Chdir("/"); err != nil { st = 1 } else { st = 0 }; // line 18
    st = 0; // line 19
    if err := os.RemoveAll(d); err != nil { st = 1 }
    _ = st
    os.Exit(int(st));
}