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
    var result1 string
    var result2 string
    var st int64

    fmt.Println("Testing system calls with builtin commands");
    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 4
    st = runCmd(exec.Command("cd", d)); // line 5
    if st != 0 {
        st = runCmd(exec.Command("exit", "1"));
    }
    os.WriteFile("a.txt", []byte("a\n"), 0644); // line 6
    os.WriteFile("b.txt", []byte("b\n"), 0644); // line 7
    st = runCmd(exec.Command("mkdir", "sub")); // line 8
    os.WriteFile("sub/c.txt", []byte("c\n"), 0644); // line 9
    result1 = s2s(capCmd(exec.Command("ls", "-A"))); // line 11
    st = 0;
    result2 = s2s(capCmd(exec.Command("find", ".", "-name", globArgs("*.txt")...)));
    st = 0;
    fmt.Println("Results:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", result1)); // line 15
    st = 0;
    fmt.Println(fmt.Sprintf("%v", result2)); // line 16
    st = 0;
    _ = os.Chdir("/"); // line 17
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 18
    _ = st
    return;
}