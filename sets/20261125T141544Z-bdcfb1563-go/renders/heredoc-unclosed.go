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

    st = redirRun("( ( 'echo' 'test' ) ) 0<<'EOF'\nfirst line\nsecond line\n\nEOF 2>&1 1>/dev/null", []string{}); // line 4
    fmt.Println("after");
    st = 0;
    fmt.Println(fmt.Sprintf("done: %v", st)); // line 10
    st = 0;
    return;
}