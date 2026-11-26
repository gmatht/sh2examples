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

    os.WriteFile("/tmp/084_test.txt", []byte("hello\n"), 0644);
    t1, _ := os.OpenFile("/tmp/084_test.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644); t1.Write([]byte("world\n")); t1.Close();
    st = redirRun("( while 'read' '-r' 'word'; do 'echo' \"$word\" | 'tr' 'a-z' 'A-Z'; done ) 0</tmp/084_test.txt", []string{});
    st = runCmd(exec.Command("rm", "-f", "/tmp/084_test.txt")); // line 9
    _ = st
    return;
}