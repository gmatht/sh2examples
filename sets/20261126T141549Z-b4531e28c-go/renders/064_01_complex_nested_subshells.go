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
    var __ps_tmp1 string
    var __ps_tmp2 string
    var __ps_tmp3 string
    var st int64

    // TODO(unsupported): capture
    __ps_tmp1 = s2s(0);
    st = 0;
    // TODO(unsupported): Redirect
    // TODO(unsupported): capture
    __ps_tmp3 = s2s(0);
    st = 0;
    // TODO(unsupported): Redirect
    st = runCmd(exec.Command("diff", __ps_tmp1, __ps_tmp3));
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp1", None), Var("__ps_tmp3", None)], capture: None, redirects: [], env: [] }
    fmt.Println(fmt.Sprintf("done: %v", st)); // line 6
    st = 0;
    _ = __ps_tmp0
    _ = __ps_tmp2
    return;
}
// 5 construct(s) lowered to TODO markers