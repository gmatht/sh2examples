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
    var current_date string
    var current_dir string

    fmt.Println("=== Basic Command Substitution ===");
    fmt.Println(fmt.Sprintf("Current date: %v", capCmd(exec.Command("date", "+%Y")))); // line 9
    fmt.Println(fmt.Sprintf("Current directory: %v", capCmd(exec.Command("basename", s2s(strings.Join(strings.Fields(capCmd(exec.Command("pwd"))), " ")))))); // line 11
    current_date = s2s(capCmd(exec.Command("date", "+%Y%m"))); // line 14
    current_dir = s2s(capCmd(exec.Command("basename", s2s(strings.Join(strings.Fields(capCmd(exec.Command("pwd"))), " "))))); // line 16
    fmt.Println(fmt.Sprintf("Stored date: %v", current_date)); // line 18
    fmt.Println(fmt.Sprintf("Stored directory: %v", current_dir)); // line 20
    fmt.Println("=== Basic Command Substitution Complete ===");
    return;
}