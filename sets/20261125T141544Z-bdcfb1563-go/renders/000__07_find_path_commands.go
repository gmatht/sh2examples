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
    var found_files string
    var st int64

    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 6
    st = runCmd(exec.Command("mkdir", "-p", fmt.Sprintf("%v/sub", d))); // line 7
    st = runCmd(exec.Command("touch", fmt.Sprintf("%v/a.sh", d), fmt.Sprintf("%v/b.sh", d), fmt.Sprintf("%v/sub/c.sh", d), fmt.Sprintf("%v/data.txt", d))); // line 8
    found_files = s2s(capRun("'cd' \"$d\" && 'find' '.' '-name' '*.sh' '-type' 'f' | 'sort'", []string{"d=" + s2s(d), "found_files=" + s2s(found_files)}));
    st = 0;
    fmt.Println("Found shell scripts:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", found_files)); // line 11
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 12
    _ = st
    return;
}