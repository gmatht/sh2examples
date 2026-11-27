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

var fArgs []string

func capRun(script string, env []string) string {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func main() {
    var upper func()
    var val any

    upper = func() { // line 4
        // declare val
        val = capRun("'echo' \"$1\" | 'tr' 'a-z' 'A-Z'", []string{"upper=" + s2s(upper), "val=" + s2s(val)});
        fmt.Println(fmt.Sprintf("%v", val));
    }
    fArgs = []string{"hello"}; upper(); fArgs = nil;
    fArgs = []string{"world"}; upper(); fArgs = nil;
    return;
}