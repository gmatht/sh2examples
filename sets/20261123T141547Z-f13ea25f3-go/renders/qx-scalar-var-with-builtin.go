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

    result = s2s(capCmd(exec.Command("sed", "-n", "1p", "/some/file"))); // line 4
    fmt.Println(fmt.Sprintf("%v", result)); // line 5
    return;
}