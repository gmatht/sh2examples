package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
    "syscall"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func b2i(b bool) int64 {
    if b { return 1 }
    return 0
}

func capCmd(c *exec.Cmd) string {
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
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

func main() {
    var __sh2_tc0 string
    var st int64

    __sh2_tc0 = s2s(capCmd(exec.Command("readlink", os.Getenv("FILE")))); // line 3
    st = b2i(!(((((!fileTest("h", s2s(os.Getenv("FILE")))) && fileTest("d", s2s(os.Getenv("FILE"))))) || ((fileTest("h", s2s(os.Getenv("FILE"))) && (s2s(__sh2_tc0) == s2s("target")))))));
    fmt.Print("parsed OK\n");
    st = 0;
    _ = st
    return;
}