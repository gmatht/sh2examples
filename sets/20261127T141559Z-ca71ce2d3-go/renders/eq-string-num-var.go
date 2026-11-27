package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strconv"
    "strings"
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

func b2i(b bool) int64 {
    if b { return 1 }
    return 0
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

func main() {
    var i int64
    var st int64

    for _, i = range []int64{s2i("1"), s2i("2"), s2i("3")} {
        st = b2i(((!(i != 2)) && (runCmd(exec.Command("echo", fmt.Sprintf("hit=%v", i))) == 0)));
    }
    st = b2i(!((s2s(st) == s2s(0))));
    if st == 0 {
        st = runCmd(exec.Command("echo", "status-zero"));
    }
    st = 0; // line 8
    st = b2i(!((s2s(st) == s2s(0))));
    if st == 0 {
        st = runCmd(exec.Command("echo", "status-zero2"));
    }
    return;
}