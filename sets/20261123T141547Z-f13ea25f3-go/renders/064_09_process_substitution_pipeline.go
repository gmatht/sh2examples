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

func runPipe(stages [][]string) int64 {
    var cmds []*exec.Cmd
    for _, st := range stages {
        c := exec.Command(st[0], st[1:]...)
        c.Stderr = io.Discard
        cmds = append(cmds, c)
    }
    if len(cmds) == 0 { return 0 }
    for i := 0; i < len(cmds)-1; i++ {
        p, err := cmds[i].StdoutPipe()
        if err != nil { return 1 }
        cmds[i+1].Stdin = p
    }
    cmds[len(cmds)-1].Stdout = os.Stdout
    for i, c := range cmds {
        if i < len(cmds)-1 {
            if err := c.Start(); err != nil { return 1 }
        } else if err := c.Run(); err != nil {
            if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
            return 1
        }
    }
    for i := 0; i < len(cmds)-1; i++ { cmds[i].Wait() }
    return 0
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
    st = runPipe([][]string{[]string{"paste", __ps_tmp0, __ps_tmp1}, []string{"head", "-10"}});
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp0", None), Var("__ps_tmp1", None)], capture: None, redirects: [], env: [] }
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 6
    st = 0;
    return;
}
// 5 construct(s) lowered to TODO markers