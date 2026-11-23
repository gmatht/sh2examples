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
    var tmpf string
    var st int64

    tmpf = fmt.Sprintf("/tmp/%v.txt", int64(os.Getpid())); // line 3
    st = redirRun("( echo hello ) 1>${tmpf}", []string{"tmpf=" + s2s(tmpf)});
    st = runCmd(exec.Command("cat", tmpf)); // line 5
    st = runCmd(exec.Command("rm", tmpf)); // line 6
    _ = st
    return;
}