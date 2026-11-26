package main

import (
    "fmt"
    "io"
    "os/exec"
    "strings"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func capCmd(c *exec.Cmd) string {
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func main() {
    var result string
    var st int64

    result = s2s(capCmd(exec.Command("grep", "pattern", "file.txt"))); // line 1
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 3
    st = 0;
    _ = result
    return;
}