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
    var d string
    var tee_result string
    var st int64

    fmt.Println("=== Output and Formatting Commands ===");
    fmt.Println(fmt.Sprintf("Echo result: %v", "Hello from backticks"));
    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 15
    st = runCmd(exec.Command("cd", d)); // line 16
    if st != 0 {
        st = runCmd(exec.Command("exit", "1"));
    }
    fmt.Println(fmt.Sprintf("Printf result: %v", "Number: 42, String: test"));
    st = 0;
    tee_result = s2s(capRun("'echo' 'test output' | 'tee' 'test_tee.txt'", []string{"d=" + s2s(d), "tee_result=" + s2s(tee_result)}));
    st = 0;
    fmt.Println(fmt.Sprintf("Tee result: %v", tee_result)); // line 23
    st = 0;
    st = runCmd(exec.Command("rm", "-f", "test_tee.txt")); // line 26
    fmt.Println("=== Output and Formatting Commands Complete ===");
    st = 0;
    _ = os.Chdir("/"); // line 29
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 30
    _ = st
    return;
}