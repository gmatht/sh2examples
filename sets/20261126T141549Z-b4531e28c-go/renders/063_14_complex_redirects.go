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

func redirRun(script string, env []string) int64 {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    return runCmd(c)
}

func main() {
    var __ps_tmp0 string
    var __ps_tmp1 string
    var st int64

    // TODO(unsupported): capture
    __ps_tmp0 = s2s(0);
    st = 0;
    // TODO(unsupported): Redirect
    // TODO(unsupported): capture
    __ps_tmp1 = s2s(0);
    st = 0;
    // TODO(unsupported): Redirect
    st = redirRun("( 'diff' \"$__ps_tmp0\" \"$__ps_tmp1\" ) 1>comparison.txt 2>&1", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "__ps_tmp1=" + s2s(__ps_tmp1)});
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp0", None), Var("__ps_tmp1", None)], capture: None, redirects: [], env: [] }
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 6
    st = 0;
    _ = __ps_tmp0
    _ = __ps_tmp1
    return;
}
// 5 construct(s) lowered to TODO markers