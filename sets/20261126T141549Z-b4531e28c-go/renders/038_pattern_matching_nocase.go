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

    fmt.Println("== nocasematch ==");
    st = b2i(!(strings.EqualFold(s2s("Foo"), s2s("foo"))));
    if st == 0 {
        st = runCmd(exec.Command("echo", "ci-match"));
    }
    _ = st
    return;
}