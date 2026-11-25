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

func redirRun(script string, env []string) int64 {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    return runCmd(c)
}

func main() {
    var st int64

    st = redirRun("'echo' '/tmp/chown_through_xargs_test' | 'xargs' '-0' '--no-run-if-empty' 'echo' 'chown' '-c' 'root:root' '--' '/tmp/chown_through_xargs_test'", []string{});
    fmt.Println(fmt.Sprintf("status: %v", "ok"));
    st = 0;
    _ = st
    return;
}