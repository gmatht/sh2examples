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
    var var_ any
    var st int64

    for _, var_ = range []any{"BASH_ENV", "ENV", "MAIL"} { // line 4
        st = runCmd(exec.Command("eval", "test", fmt.Sprintf("\\${%v+y}", var_)));
        if st == 0 {
            st = runCmd(exec.Command("unset", s2s(strings.Join(strings.Fields(s2s(var_)), " "))));
        }
        if st != 0 {
            st = runCmd(exec.Command(":"));
        }
    }
    fmt.Print("parsed OK\n");
    st = 0;
    _ = st
    return;
}