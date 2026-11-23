package main

import (
    "fmt"
    "io"
    "os"
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

func capRun(script string, env []string) string {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func main() {
    var formatted_date string
    var yes_result string

    fmt.Println("=== System Utilities ===");
    formatted_date = s2s(capCmd(exec.Command("date", "+%Y-%m-%d"))); // line 11
    fmt.Println(fmt.Sprintf("Formatted date: %v", formatted_date)); // line 13
    yes_result = s2s(capRun("'yes' 'Hello' | 'head' '-3'", []string{"formatted_date=" + s2s(formatted_date), "yes_result=" + s2s(yes_result)}));
    fmt.Println("Yes command result:");
    fmt.Println(fmt.Sprintf("%v", yes_result)); // line 35
    return;
}