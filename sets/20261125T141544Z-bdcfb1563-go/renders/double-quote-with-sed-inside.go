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
    var VERSION string

    VERSION = s2s(capRun("'grep' 'VERSION_STR' '/some/file' | 'awk' '{ print $2 }' | 'sed' 's/\"//g'", []string{"VERSION=" + s2s(VERSION)})); // line 3
    fmt.Println(fmt.Sprintf("%v", VERSION)); // line 4
    return;
}