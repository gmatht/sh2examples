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

    for {
        fmt.Println("checking...");
        st = 0;
        st = runCmd(exec.Command("test", "-f", "/tmp/somefile"));
        if st != 0 { break; }
        fmt.Println("file exists");
        st = 0;
        break;
    }
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 11
    st = 0;
    return;
}