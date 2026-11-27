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

func runPipe(stages [][]string) int64 {
    var cmds []*exec.Cmd
    for _, st := range stages {
        c := exec.Command(st[0], st[1:]...)
        c.Stderr = io.Discard
        cmds = append(cmds, c)
    }
    if len(cmds) == 0 { return 0 }
    for i := 0; i < len(cmds)-1; i++ {
        p, err := cmds[i].StdoutPipe()
        if err != nil { return 1 }
        cmds[i+1].Stdin = p
    }
    cmds[len(cmds)-1].Stdout = os.Stdout
    for i, c := range cmds {
        if i < len(cmds)-1 {
            if err := c.Start(); err != nil { return 1 }
        } else if err := c.Run(); err != nil {
            if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
            return 1
        }
    }
    for i := 0; i < len(cmds)-1; i++ { cmds[i].Wait() }
    return 0
}

func globArgs(pat string) []string {
    m, err := filepath.Glob(pat)
    if err != nil || m == nil { return []string{pat} }
    return m
}

func main() {
    var d string
    var st int64

    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 4
    st = runCmd(exec.Command("cd", d)); // line 5
    if st != 0 {
        st = runCmd(exec.Command("exit", "1"));
    }
    os.WriteFile("file.txt", []byte("apple\napple\n"), 0644); // line 6
    os.WriteFile("note.txt", []byte("hello.txt\n"), 0644); // line 7
    os.WriteFile("s.sh", []byte("function f() { echo hi; }\n"), 0644); // line 8
    st = runPipe([][]string{[]string{"ls"}, []string{"grep", "\\.txt$"}, []string{"wc", "-l"}});
    fmt.Println(""); // line 10
    st = 0;
    st = runPipe([][]string{[]string{"cat", "file.txt"}, []string{"sort"}, []string{"uniq", "-c"}, []string{"sort", "-n", "-r"}}); // line 11
    fmt.Println(""); // line 12
    st = 0;
    st = runPipe([][]string{[]string{"find", ".", "-name", globArgs("*.sh")...}, []string{"xargs", "grep", "-l", "function"}, []string{"tr", "-d", "\\\\/"}});
    fmt.Println(""); // line 14
    st = 0;
    st = runPipe([][]string{[]string{"cat", "file.txt"}, []string{"tr", "a", "b"}, []string{"grep", "hello"}});
    fmt.Println(""); // line 16
    st = 0;
    st = runPipe([][]string{[]string{"cat", "file.txt"}, []string{"sort"}, []string{"grep", "hello"}}); // line 17
    _ = os.Chdir("/"); // line 18
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 19
    _ = st
    return;
}