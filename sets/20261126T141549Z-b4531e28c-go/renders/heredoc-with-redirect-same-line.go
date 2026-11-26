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
    var tmpf string
    var st int64

    tmpf = s2s(capCmd(exec.Command("mktemp", "/tmp/heredoc_with_redirect_same_line.XXXXXX"))); // line 2
    fmt.Print("hello world\n"); // line 3
    fmt.Print(fmt.Sprintf("heredoc+same-line-redirect content=[%v]\n", capCmd(exec.Command("cat", tmpf)))); // line 6
    st = runCmd(exec.Command("rm", "-f", tmpf)); // line 7
    _ = st
    return;
}