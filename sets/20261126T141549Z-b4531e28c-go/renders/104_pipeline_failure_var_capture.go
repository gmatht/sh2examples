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
    var count string
    var d string
    var files string
    var st int64

    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 3
    files = s2s(capRun("( 'ls' \"$d\" ) 2>/dev/null | 'wc' '-l'", []string{"count=" + s2s(count), "d=" + s2s(d), "files=" + s2s(files)})); // line 4
    fmt.Println(fmt.Sprintf("Files: %v", files)); // line 5
    count = s2s(capRun("'echo' 'a b c' | 'wc' '-w'", []string{"count=" + s2s(count), "d=" + s2s(d), "files=" + s2s(files)}));
    fmt.Println(fmt.Sprintf("Words: %v", count)); // line 7
    _ = os.Chdir("/"); // line 8
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 9
    fmt.Println("done");
    st = 0;
    _ = st
    return;
}