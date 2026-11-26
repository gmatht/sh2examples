package main

import (
    "fmt"
    "os"
    "syscall"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
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

func fileNewer(a, b string, nt bool) bool {
    sa, ea := os.Stat(a)
    sb, eb := os.Stat(b)
    if ea != nil || eb != nil { return false }
    if nt { return sa.ModTime().After(sb.ModTime()) }
    return sb.ModTime().After(sa.ModTime())
}

func fileSame(a, b string) bool {
    sa, ea := os.Stat(a)
    sb, eb := os.Stat(b)
    if ea != nil || eb != nil { return false }
    return os.SameFile(sa, sb)
}

func main() {
    if fileTest("h", s2s("/dev/null")) { // line 3
    }
    if fileTest("p", s2s("/dev/null")) { // line 4
    }
    if fileTest("b", s2s("/dev/null")) { // line 5
    }
    if fileTest("c", s2s("/dev/null")) { // line 6
    }
    if fileTest("g", s2s("/dev/null")) { // line 7
    }
    if fileTest("k", s2s("/dev/null")) { // line 8
    }
    if fileTest("u", s2s("/dev/null")) { // line 9
    }
    if fileTest("O", s2s("/dev/null")) { // line 10
    }
    if fileTest("G", s2s("/dev/null")) { // line 11
    }
    if fileTest("N", s2s("/dev/null")) { // line 12
    }
    if fileNewer(s2s("a"), s2s("b"), true) { // line 14
    }
    if fileNewer(s2s("a"), s2s("b"), false) { // line 15
    }
    if fileSame(s2s("a"), s2s("b")) { // line 16
    }
    if true {
    }
    if true {
    }
    fmt.Print("parsed OK\n");
    fmt.Println(fmt.Sprintf("status: %v", "ok"));
    return;
}