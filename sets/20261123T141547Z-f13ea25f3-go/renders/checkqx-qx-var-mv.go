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

    st = runCmd(exec.Command("touch", "src.txt", "dest.txt")); // line 4
    st = redirRun("( 'mv' '-Z' 'src.txt' 'dest.txt' ) 2>/dev/null", []string{}); // line 5
    fmt.Println(fmt.Sprintf("mv rc: %v", st)); // line 6
    st = 0;
    st = runCmd(exec.Command("rm", "-f", "src.txt", "dest.txt")); // line 7
    return;
}