package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
)

var fArgs []string

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1 { return os.Args[1:] }
    return nil
}

func paramAt(i int) string {
    a := argsList()
    if i >= 1 && i <= len(a) { return a[i-1] }
    return ""
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

func main() {
    var st int64

    st = runCmd(exec.Command("cd", fmt.Sprintf("%v/../..", capCmd(exec.Command("dirname", paramAt(0))))));
    if st != 0 {
        st = runCmd(exec.Command("die", "Could not cd"));
    }
    fmt.Print("parsed OK\n");
    st = 0;
    _ = st
    return;
}