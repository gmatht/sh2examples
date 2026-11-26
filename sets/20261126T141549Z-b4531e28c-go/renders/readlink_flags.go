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
    var existing string
    var full string
    var missing string

    existing = s2s(capCmd(exec.Command("readlink", "-e", "/usr/bin/vi"))); // line 3
    missing = s2s(capCmd(exec.Command("readlink", "-m", "/nonexistent/path"))); // line 4
    full = s2s(capCmd(exec.Command("readlink", "-f", "/usr/bin/python3"))); // line 5
    fmt.Println(fmt.Sprintf("Existing: %v", existing)); // line 6
    fmt.Println(fmt.Sprintf("Missing:  %v", missing)); // line 7
    fmt.Println(fmt.Sprintf("Full:     %v", full)); // line 8
    return;
}