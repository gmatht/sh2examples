package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
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
    var st int64

    st = b2i(!((s2s("~") == s2s(os.Getenv("HOME"))))); // line 1
    if st == 0 {
        st = runCmd(exec.Command("echo", "1"));
    }
    if st != 0 {
        st = runCmd(exec.Command("echo", "-"));
    }
    st = b2i(!((s2s("~/Documents") == s2s(os.Getenv("HOME"))))); // line 2
    if st == 0 {
        st = runCmd(exec.Command("echo", "2"));
    }
    if st != 0 {
        st = runCmd(exec.Command("echo", "-"));
    }
    st = b2i(!((s2s("~/Documents") == s2s(os.Getenv("HOME/Documents"))))); // line 3
    if st == 0 {
        st = runCmd(exec.Command("echo", "3"));
    }
    if st != 0 {
        st = runCmd(exec.Command("echo", "-"));
    }
    _ = st
    return;
}