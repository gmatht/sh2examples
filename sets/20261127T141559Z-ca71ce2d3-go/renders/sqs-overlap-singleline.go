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
    var result string

    result = s2s(capRun("'echo' 'test' | 'awk' '{print$3}' | 'sed' 's|\\(.*\\)/.*|\\1|'", []string{"result=" + s2s(result)}));
    fmt.Println(fmt.Sprintf("%v", result)); // line 6
    return;
}