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
    var relative string

    relative = s2s(capCmd(exec.Command("readlink", "-f", "/usr/bin/corepack"))); // line 3
    fmt.Println(fmt.Sprintf("Corepack resolves to: %v", relative)); // line 4
    return;
}