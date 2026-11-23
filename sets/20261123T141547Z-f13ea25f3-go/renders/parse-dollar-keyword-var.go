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

func main() {
    var st int64

    st = runCmd(exec.Command("test", "-f", s2s(strings.Join(strings.Fields("/usr/sbin/dkms"), " "))));
    if st != 0 {
        st = runCmd(exec.Command("exit", "0"));
    }
    fmt.Print("prog=[dkms]\n");
    st = 0;
    _ = st
    return;
}