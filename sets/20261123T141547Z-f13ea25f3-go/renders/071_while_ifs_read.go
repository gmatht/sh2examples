package main

import (
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

    os.WriteFile("/tmp/while_test.txt", []byte("a\n"), 0644); // line 3
    t1, _ := os.OpenFile("/tmp/while_test.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644); t1.Write([]byte("b\n")); t1.Close(); // line 4
    st = redirRun("( while 'read' '-r' 'line'; do echo Line: $line; done ) 0</tmp/while_test.txt", []string{}); // line 5
    st = runCmd(exec.Command("rm", "-f", "/tmp/while_test.txt")); // line 8
    _ = st
    return;
}