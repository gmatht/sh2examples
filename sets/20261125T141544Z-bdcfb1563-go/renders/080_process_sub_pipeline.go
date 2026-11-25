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

func main() {
    var st int64

    os.WriteFile("/tmp/paste_a.txt", []byte("a\n"), 0644); // line 3
    os.WriteFile("/tmp/paste_b.txt", []byte("b\n"), 0644); // line 4
    st = runCmd(exec.Command("paste", "/tmp/paste_a.txt", "/tmp/paste_b.txt")); // line 5
    st = runCmd(exec.Command("rm", "-f", "/tmp/paste_a.txt", "/tmp/paste_b.txt")); // line 6
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 8
    st = 0;
    return;
}