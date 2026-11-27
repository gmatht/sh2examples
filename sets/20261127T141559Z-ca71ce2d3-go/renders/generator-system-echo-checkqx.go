package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
)

var fArgs []string

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1 { return os.Args[1:] }
    return nil
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
    var INFO func()
    var st int64

    INFO = func() { // line 3
        st = runCmd(exec.Command("/bin/echo", "-e", fmt.Sprintf("hello %v", strings.Join(argsList(), " "))));
    }
    fArgs = []string{"test"}; INFO(); fArgs = nil;
    _ = st
    return;
}