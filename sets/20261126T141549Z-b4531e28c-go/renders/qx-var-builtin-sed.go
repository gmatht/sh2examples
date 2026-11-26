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
    var all_interfaces string

    all_interfaces = s2s(capRun("( 'sed' '-n' 's/.*//p' ) 0</dev/null", []string{"all_interfaces=" + s2s(all_interfaces)})); // line 3
    fmt.Println(fmt.Sprintf("%v", all_interfaces)); // line 4
    return;
}