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

func b2i(b bool) int64 {
    if b { return 1 }
    return 0
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
    var __ps_tmp2 string
    var __ps_tmp3 string
    var st int64

    fmt.Println("== More process substitution examples ==");
    // TODO(unsupported): redirect
    // TODO(unsupported): redirect
    st = b2i(((0 == 0) || (runCmd(exec.Command("echo", "Files differ")) == 0)));
    // TODO(unsupported): capture
    __ps_tmp2 = s2s(0);
    st = 0;
    os.WriteFile(__ps_tmp2, []byte("name1\nname2\n"), 0644);
    // TODO(unsupported): capture
    __ps_tmp3 = s2s(0);
    st = 0;
    os.WriteFile(__ps_tmp3, []byte("value1\nvalue2\n"), 0644);
    st = runCmd(exec.Command("paste", __ps_tmp2, __ps_tmp3));
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp2", None), Var("__ps_tmp3", None)], capture: None, redirects: [], env: [] }
    fmt.Println(fmt.Sprintf("status: %v", "ok"));
    st = 0;
    _ = st
    return;
}
// 5 construct(s) lowered to TODO markers