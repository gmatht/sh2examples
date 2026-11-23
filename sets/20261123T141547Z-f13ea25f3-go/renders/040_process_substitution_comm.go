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
    var st int64

    fmt.Println("== Process substitution with comm ==");
    st = 0;
    // TODO(unsupported): capture
    __ps_tmp0 = s2s(0);
    st = 0;
    os.WriteFile(__ps_tmp0, []byte("a\nb\n"), 0644);
    // TODO(unsupported): capture
    __ps_tmp1 = s2s(0);
    st = 0;
    os.WriteFile(__ps_tmp1, []byte("b\nc\n"), 0644);
    st = runCmd(exec.Command("comm", "-12", __ps_tmp0, __ps_tmp1));
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp0", None), Var("__ps_tmp1", None)], capture: None, redirects: [], env: [] }
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 9
    st = 0;
    return;
}
// 3 construct(s) lowered to TODO markers