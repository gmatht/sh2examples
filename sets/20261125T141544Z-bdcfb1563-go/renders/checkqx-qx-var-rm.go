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

    st = runCmd(exec.Command("rm", "--preserve-root", "testfile")); // line 3
    fmt.Print("parsed OK\n");
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 6
    st = 0;
    return;
}