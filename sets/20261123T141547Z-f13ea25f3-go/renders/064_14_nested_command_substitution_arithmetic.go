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

    result = s2s(capRun("'echo' \"$(( $(wc -l < /etc/passwd) + $(wc -l < /etc/group) ))\"", []string{"result=" + s2s(result)})); // line 4
    fmt.Println(fmt.Sprintf("Total lines: %v", result)); // line 5
    return;
}