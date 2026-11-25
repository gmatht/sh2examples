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
    var target string
    var target2 string

    target = s2s(capCmd(exec.Command("readlink", "-f", "/usr/bin/vi"))); // line 3
    fmt.Println(fmt.Sprintf("vi resolves to: %v", target)); // line 4
    target2 = s2s(capCmd(exec.Command("readlink", "-f", "/usr/bin/python3"))); // line 5
    fmt.Println(fmt.Sprintf("python3 resolves to: %v", target2)); // line 6
    return;
}