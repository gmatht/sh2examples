package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "sort"
    "strconv"
    "strings"
)


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
    var d string
    var file_list string
    var found_files string
    var st int64

    fmt.Println("=== File and Directory Operations ===");
    st = 0;
    d = func() string { __mknt1, __mket1 := os.MkdirTemp("", "tmp*"); if __mket1 != nil { return "" }; return __mknt1 }(); // line 4
    st = 0;
    if err := os.Chdir(d); err != nil { st = 1 } else { st = 0 }; // line 5
    if st != 0 {
        os.Exit(int(1));
    }
    if __wet2 := os.WriteFile("a.sh", []byte(bprintfStr("a\\n")), 0644); __wet2 != nil { st = 1 } else { st = 0 } // line 6
    if __wet3 := os.WriteFile("b.txt", []byte(bprintfStr("b\\n")), 0644); __wet3 != nil { st = 1 } else { st = 0 } // line 7
    st = 0; // line 8
    if err := os.Mkdir("sub", 0o777); err != nil { st = 1 }
    if __wet4 := os.WriteFile("sub/c.sh", []byte(bprintfStr("c\\n")), 0644); __wet4 != nil { st = 1 } else { st = 0 } // line 9
    if __wet5 := os.WriteFile(".hidden", []byte(bprintfStr("x\\n")), 0644); __wet5 != nil { st = 1 } else { st = 0 } // line 10
    file_list = func() string { var __lsbt6 strings.Builder; st = 0;; __lsft6 := []string{}; __lsdt6 := []string{}; for _, __lsxt6 := range []string{"."} {;     __lsit6, __lset6 := os.Stat(__lsxt6);     if __lset6 != nil { st = 2 } else if __lsit6.IsDir() && true { __lsdt6 = append(__lsdt6, __lsxt6) } else { __lsft6 = append(__lsft6, __lsxt6) }; }; sort.Strings(__lsft6); sort.Strings(__lsdt6); for _, __lsxt6 := range __lsft6 { fmt.Fprintln(&__lsbt6, __lsxt6) }; for __lskt6, __lsxt6 := range __lsdt6 {;     if len(__lsft6)+len(__lsdt6) > 1 { if __lskt6 > 0 || len(__lsft6) > 0 { fmt.Fprintln(&__lsbt6, "") }; fmt.Fprintln(&__lsbt6, __lsxt6 + ":") };     __lsrt6, __lsret6 := os.ReadDir(__lsxt6);     if __lsret6 != nil { st = 2; continue };     __lsnt6 := []string{};     for _, __lset6 := range __lsrt6 {;         __lsnmt6 := __lset6.Name();         __lsnt6 = append(__lsnt6, __lsnmt6);     };     sort.Strings(__lsnt6);     for _, __lsnmt6 := range __lsnt6 { fmt.Fprintln(&__lsbt6, __lsnmt6) }; } ; return strings.TrimRight(__lsbt6.String(), "\n") }(); // line 12
    st = 0;
    fmt.Println("File listing:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", file_list)); // line 14
    st = 0;
    found_files = func() string { __cc7 := exec.Command("find", ".", "-name", "*.sh", "-type", "f"); shStdIn(__cc7); shStdErr(__cc7); __co7, _ := __cc7.Output(); return strings.TrimRight(string(__co7), "\n") }();
    st = 0;
    fmt.Println("Found shell scripts:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", found_files)); // line 18
    st = 0;
    if err := os.Chdir("/"); err != nil { st = 1 } else { st = 0 }; // line 20
    st = 0; // line 21
    if err := os.RemoveAll(d); err != nil { st = 1 }
    fmt.Println("=== File and Directory Operations Complete ===");
    st = 0;
    _ = st
    os.Exit(int(st));
}