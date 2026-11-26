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

func globArgs(pat string) []string {
    m, err := filepath.Glob(pat)
    if err != nil || m == nil { return []string{pat} }
    return m
}

func main() {
    var d string
    var st int64

    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 6
    st = 0;
    st = runCmd(exec.Command("cd", d)); // line 7
    if st != 0 {
        st = runCmd(exec.Command("exit", "1"));
    }
    st = runCmd(exec.Command("touch", "a.sh", "b.sh", ".sh")); // line 8
    fmt.Println("Testing ls * .sh:");
    st = 0;
    st = runCmd(exec.Command("ls", globArgs("*")..., ".sh")); // line 10
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 11
    st = 0;
    _ = os.Chdir("/"); // line 12
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 12
    return;
}