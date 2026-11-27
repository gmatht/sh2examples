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

    if (runCmd(exec.Command("test", "-t", "1")) == 0) {
        fmt.Println("stdout is a terminal");
        st = 0;
    }
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 7
    st = 0;
    return;
}