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

func capRun(script string, env []string) string {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func main() {
    var f any

    t1 := strings.Fields(capRun("'cmd' | 'awk' '{print $1}'", []string{"f=" + s2s(f)})); // line 6
    for _, t2 := range t1 {
        f = t2;
        fmt.Println(fmt.Sprintf("%v", f));
    }
    return;
}