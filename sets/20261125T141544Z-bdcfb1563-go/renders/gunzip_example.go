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

    st = redirRun("( 'gunzip' ) 0</usr/share/man/uk/man1/w.1.gz 1>/tmp/w.1", []string{}); // line 2
    fmt.Println("Decompressed man page");
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 5
    st = 0;
    return;
}