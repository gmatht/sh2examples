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
    var greeting any
    var x string

    greeting = "hi";
    x = s2s(capRun("'echo' \"$greeting\"; 'echo' 'second'", []string{"greeting=" + s2s(greeting), "x=" + s2s(x)})); // line 8
    fmt.Println(fmt.Sprintf("%v", x)); // line 9
    fmt.Print(fmt.Sprintf("%v\n", x)); // line 10
    _ = greeting
    return;
}