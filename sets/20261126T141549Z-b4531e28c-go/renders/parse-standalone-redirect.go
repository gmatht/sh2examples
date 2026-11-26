package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
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

func capRun(script string, env []string) string {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
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

    st = redirRun("( '' ) 1>somefile", []string{}); // line 3
    fmt.Print(fmt.Sprintf("standalone redirect file size=%v\n", capRun("( 'stat' '-c%s' 'somefile' ) 2>/dev/null || ( 'wc' '-c' ) 0<somefile", []string{}))); // line 4
    st = 0;
    st = runCmd(exec.Command("rm", "somefile")); // line 5
    _ = st
    return;
}