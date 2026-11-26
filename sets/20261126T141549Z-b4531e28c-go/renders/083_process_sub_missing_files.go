package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
)

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

    fmt.Println("start");
    st = 0;
    // TODO(unsupported): redirect
    // TODO(unsupported): redirect
    st = b2i(((0 == 0) || (runCmd(exec.Command("true")) == 0)));
    fmt.Println("end");
    st = 0;
    fmt.Println(fmt.Sprintf("done: %v", st)); // line 9
    st = 0;
    return;
}
// 2 construct(s) lowered to TODO markers