package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "path/filepath"
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

func globArgs(pat string) []string {
    m, err := filepath.Glob(pat)
    if err != nil || m == nil { return []string{pat} }
    return m
}

func main() {
    var st int64

    fmt.Println("== Practical examples ==");
    st = 0;
    st = runCmd(exec.Command("touch", "file_001.txt", "file_002.txt", "file_003.txt", "file_004.txt", "file_005.txt"));
    st = runCmd(exec.Command("ls", globArgs("file_*.txt")...)); // line 9
    st = runCmd(exec.Command("rm", globArgs("file_*.txt")...)); // line 10
    fmt.Println(fmt.Sprintf("done: %v", st)); // line 12
    st = 0;
    return;
}