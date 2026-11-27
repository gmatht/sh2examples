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
    var file_list string
    var found_files string
    var st int64

    fmt.Println("=== File and Directory Operations ===");
    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 4
    st = runCmd(exec.Command("cd", d)); // line 5
    if st != 0 {
        st = runCmd(exec.Command("exit", "1"));
    }
    os.WriteFile("a.sh", []byte("a\n"), 0644); // line 6
    os.WriteFile("b.txt", []byte("b\n"), 0644); // line 7
    st = runCmd(exec.Command("mkdir", "sub")); // line 8
    os.WriteFile("sub/c.sh", []byte("c\n"), 0644); // line 9
    os.WriteFile(".hidden", []byte("x\n"), 0644); // line 10
    file_list = s2s(capCmd(exec.Command("ls", "-A"))); // line 12
    st = 0;
    fmt.Println("File listing:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", file_list)); // line 14
    st = 0;
    found_files = s2s(capCmd(exec.Command("find", ".", "-name", globArgs("*.sh")..., "-type", "f")));
    st = 0;
    fmt.Println("Found shell scripts:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", found_files)); // line 18
    st = 0;
    _ = os.Chdir("/"); // line 20
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 21
    fmt.Println("=== File and Directory Operations Complete ===");
    st = 0;
    _ = st
    return;
}