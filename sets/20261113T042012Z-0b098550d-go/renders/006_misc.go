package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
)


// pipeStatus is bash's PIPESTATUS: the exit code of EVERY stage of the
// last pipeline (not just the last one).
var pipeStatus []int64

func exitCode(err error) int64 {
    if err == nil { return 0 }
    if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
    return 1
}

func runPipe(stages [][]string) int64 {
    var cmds []*exec.Cmd
    for _, st := range stages {
        c := exec.Command(st[0], st[1:]...)
        cmds = append(cmds, c)
    }
    if len(cmds) == 0 { return 0 }
    var pipes []io.Closer
    for i := 0; i < len(cmds)-1; i++ {
        p, err := cmds[i].StdoutPipe()
        if err != nil { return 1 }
        cmds[i+1].Stdin = p
        pipes = append(pipes, p)
    }
    cmds[len(cmds)-1].Stdout = os.Stdout
    // After the pipe wiring (StdoutPipe refuses an already-set Stdout):
    // unwired Stdin ends follow the current process stdin.
    for _, c := range cmds { if c.Stdin == nil { c.Stdin = os.Stdin } }
    codes := make([]int64, len(cmds))
    last := int64(0)
    for i, c := range cmds {
        if i < len(cmds)-1 {
            if err := c.Start(); err != nil { codes[i] = 1 }
        } else {
            last = exitCode(c.Run())
            codes[i] = last
        }
    }
    // The parent holds a read end of every inter-stage pipe: close
    // the parent copies after Run so an early-exiting downstream
    // stage (head) retires the producers instead of deadlocking them.
    for _, p := range pipes { p.Close() }
    for i := 0; i < len(cmds)-1; i++ {
        codes[i] = exitCode(cmds[i].Wait())
    }
    pipeStatus = codes
    return last
}
func main() {
    var st int64

    fmt.Println("== Subshell ==");
    st = 0;
    fmt.Println("inside-subshell"); // line 4
    st = 0;
    fmt.Println("== Simple pipeline ==");
    st = 0;
    st = runPipe([][]string{[]string{"echo", "alpha beta"}, []string{"grep", "beta"}});
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 11
    st = 0;
    os.Exit(int(st));
}