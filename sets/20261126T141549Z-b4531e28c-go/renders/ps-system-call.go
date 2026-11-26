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

    if (redirRun("( 'ps' '-C' 'apt-get,apt,dpkg' ) >/dev/null 2>&1", []string{}) == 0) {
        fmt.Println("Package manager running");
        st = 0;
    }
    fmt.Println(fmt.Sprintf("done: %v", st)); // line 8
    st = 0;
    return;
}