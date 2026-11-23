package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

var fArgs []string

func runCmd(c *exec.Cmd) int64 {
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    if err := c.Run(); err != nil {
        if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
        return 1
    }
    return 0
}

func redirRun(script string, env []string) int64 {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    return runCmd(c)
}

func main() {
    var f func()
    var g func()
    var st int64

    g = func() { // line 28
        st = redirRun("( echo hi ) 2>/dev/null", []string{"f=" + s2s(f), "g=" + s2s(g)});
    }
    f = func() { // line 29
        fArgs = []string{}; g(); fArgs = nil;
    }
    fArgs = []string{}; f(); fArgs = nil; // line 30
    _ = st
    return;
}