package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
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

func main() {
    var st int64

    if ((!(paramAt(1) != "-h")) || (!(paramAt(1) != "--help"))) {
        st = runCmd(exec.Command("usage"));
        os.Exit(int(0));
    } else if (!(paramAt(1) != "--version")) {
        fmt.Println(fmt.Sprintf("%v", os.Getenv("VERSION")));
        st = 0;
        os.Exit(int(0));
    }
    _ = st
    return;
}