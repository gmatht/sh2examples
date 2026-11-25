package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
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

func capCmd(c *exec.Cmd) string {
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func main() {
    var d string
    var st int64

    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 4
    st = runCmd(exec.Command("touch", fmt.Sprintf("%v/a", d), fmt.Sprintf("%v/b", d))); // line 5
    st = runCmd(exec.Command("ln", fmt.Sprintf("%v/a", d), fmt.Sprintf("%v/a2", d))); // line 6
    st = runCmd(exec.Command("test", fmt.Sprintf("%v/a", d), "-ef", fmt.Sprintf("%v/a2", d)));
    if st == 0 {
        st = runCmd(exec.Command("echo", "same file"));
    }
    st = runCmd(exec.Command("test", fmt.Sprintf("%v/a", d), "-ef", fmt.Sprintf("%v/b", d)));
    if st != 0 {
        st = runCmd(exec.Command("echo", "different files"));
    }
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 9
    _ = st
    return;
}