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
    var show_help func()
    var st int64

    show_help = func() { // line 3
        st = redirRun("( 'cat' ) 0<<-'EOF'\nUsage: script [options]\n\nEOF", []string{"show_help=" + s2s(show_help)});
    }
    fArgs = []string{}; show_help(); fArgs = nil; // line 8
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 10
    st = 0;
    return;
}