package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
)

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

    st = 0;
    _ = (!(runCmd(exec.Command("grep", "-q", "foo", "/dev/null")) == 0)); // line 7
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 9
    st = 0;
    return;
}