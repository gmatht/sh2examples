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

    if (!(redirRun("( 'command' '-v' 'nonexistent' ) >/dev/null", []string{}) == 0)) {
        fmt.Println("not found");
        st = 0;
    }
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 7
    st = 0;
    return;
}