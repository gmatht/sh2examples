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
    var x string

    x = s2s(capRun("( 'cat' ) 0<<'EOF'\nline1\nline2\n\nEOF", []string{"x=" + s2s(x)})); // line 3
    fmt.Print(fmt.Sprintf("result=[%v]\n", x)); // line 8
    return;
}