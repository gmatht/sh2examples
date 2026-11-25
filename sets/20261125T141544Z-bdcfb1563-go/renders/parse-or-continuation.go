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
    var die func()
    var st int64

    die = func() { // line 3
        st = redirRun("( 'echo' \"$@\" ) 1>&2", []string{"die=" + s2s(die)});
        os.Exit(int(1));
    }
    st = runCmd(exec.Command("cd", "/tmp"));
    if st != 0 {
        st = runCmd(exec.Command("die", "Could not cd"));
    }
    _ = die
    _ = st
    return;
}