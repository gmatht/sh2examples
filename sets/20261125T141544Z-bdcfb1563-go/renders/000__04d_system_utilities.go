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

func runCmd(c *exec.Cmd) int64 {
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    if err := c.Run(); err != nil {
        if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
        return 1
    }
    return 0
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
    var st int64

    fmt.Println("=== System Utilities ===");
    formatted_date = s2s(capCmd(exec.Command("date", "+%Y-%m-%d"))); // line 11
    fmt.Println(fmt.Sprintf("Formatted date: %v", formatted_date)); // line 13
    fmt.Println(fmt.Sprintf("Sleeping for %v seconds...", "1"));
    st = runCmd(exec.Command("sleep", "1"));
    yes_result = s2s(capRun("'yes' 'Hello' | 'head' '-3'", []string{"formatted_date=" + s2s(formatted_date), "yes_result=" + s2s(yes_result)}));
    st = 0;
    fmt.Println("Yes command result:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", yes_result)); // line 35
    st = 0;
    fmt.Println("=== System Utilities Complete ===");
    st = 0;
    _ = st
    return;
}