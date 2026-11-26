package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
)

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

    st = runCmd(exec.Command("bash", "examples/005_args.sh", "one")); // line 1
    st = runCmd(exec.Command("bash", "examples/005_args.sh", "one", "two")); // line 2
    st = runCmd(exec.Command("bash", "examples/005_args.sh", "one", "two", "three")); // line 3
    st = runCmd(exec.Command("bash", "examples/005_args.sh", "1")); // line 4
    st = runCmd(exec.Command("bash", "examples/005_args.sh", "1", "2", "3")); // line 5
    st = runCmd(exec.Command("bash", "examples/005_args.sh", "1", "two", "3")); // line 6
    st = runCmd(exec.Command("bash", "examples/005_args.sh", "A 'quoted' Sting"));
    st = runCmd(exec.Command("bash", "examples/005_args.sh", "A 'quoted' Sting", "2", "3", "4", "5", "6"));
    fmt.Println(fmt.Sprintf("status: %v", "ok"));
    st = 0;
    _ = st
    return;
}