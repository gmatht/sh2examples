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
    var st int64

    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 3
    st = runCmd(exec.Command("mkdir", "-p", fmt.Sprintf("%v/a", d))); // line 4
    st = runCmd(exec.Command("cd", fmt.Sprintf("%v/a", d))); // line 5
    if st != 0 {
        st = runCmd(exec.Command("exit", "1"));
    }
    _ = os.Chdir(".."); // line 6
    st = 0;
    fmt.Print(fmt.Sprintf("parent contains a: %v\n", capRun("'test' '-d' 'a' && 'echo' 'yes' || 'echo' 'no'", []string{"d=" + s2s(d)}))); // line 7
    st = 0;
    _ = os.Chdir("/"); // line 8
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 9
    _ = st
    return;
}