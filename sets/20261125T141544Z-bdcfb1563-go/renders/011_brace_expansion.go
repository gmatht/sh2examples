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

    fmt.Println("== Basic brace expansion ==");
    fmt.Println("1 2 3 4 5");
    fmt.Println("a b c");
    fmt.Println("00 02 04");
    fmt.Println("== Advanced brace expansion ==");
    fmt.Println("a1 a2 a3 b1 b2 b3 c1 c2 c3");
    fmt.Println("1 3 5 7 9");
    fmt.Println("a d g j m p s v y");
    fmt.Println("== Practical examples ==");
    st = runCmd(exec.Command("touch", "file_001.txt", "file_002.txt", "file_003.txt", "file_004.txt", "file_005.txt"));
    st = runCmd(exec.Command("ls", globArgs("file_*.txt")...)); // line 21
    st = runCmd(exec.Command("rm", globArgs("file_*.txt")...)); // line 22
    fmt.Println(fmt.Sprintf("brace_expand: %v", "1 2 3 4 5"));
    st = 0;
    _ = st
    return;
}