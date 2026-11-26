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
    var LAST_LINE string

    LAST_LINE = s2s(capCmd(exec.Command("tail", "-1", "/some/logfile"))); // line 3
    fmt.Println(fmt.Sprintf("%v", LAST_LINE)); // line 4
    return;
}