package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
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
    var __ps_tmp0 string
    var st int64

    // TODO(unsupported): capture
    __ps_tmp0 = s2s(0);
    st = 0;
    // TODO(unsupported): Redirect
    st = runCmd(exec.Command("head", __ps_tmp0));
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp0", None)], capture: None, redirects: [], env: [] }
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 5
    st = 0;
    return;
}
// 3 construct(s) lowered to TODO markers