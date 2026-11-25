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

    os.WriteFile("/tmp/088_data.txt", []byte("b:2\n"), 0644);
    t1, _ := os.OpenFile("/tmp/088_data.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644); t1.Write([]byte("a:1\n")); t1.Close();
    t2, _ := os.OpenFile("/tmp/088_data.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644); t2.Write([]byte("c:3\n")); t2.Close();
    st = redirRun("( 'read' '-r' 'name' 'num' ) 0</tmp/088_data.txt | 'sort' '-n'", []string{}); // line 7
    st = runCmd(exec.Command("rm", "-f", "/tmp/088_data.txt")); // line 10
    _ = st
    return;
}