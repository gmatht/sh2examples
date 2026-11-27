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

    _ = os.Chdir(".."); // line 1
    st = 0;
    st = runCmd(exec.Command("ls")); // line 2
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 4
    st = 0;
    return;
}