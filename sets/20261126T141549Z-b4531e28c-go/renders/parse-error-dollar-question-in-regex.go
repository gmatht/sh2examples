package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "path/filepath"
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

func globArgs(pat string) []string {
    m, err := filepath.Glob(pat)
    if err != nil || m == nil { return []string{pat} }
    return m
}

func main() {
    var cmd string

    cmd = s2s(capCmd(exec.Command("grep", "-E", globArgs("^Exec(\\[[^]=]*])?=")..., os.Getenv("file"))));
    fmt.Println(fmt.Sprintf("%v", cmd)); // line 4
    return;
}