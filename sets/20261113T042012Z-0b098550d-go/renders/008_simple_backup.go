package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
    "time"
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

func capPipe(stages [][]string) string {
    return strings.TrimRight(capPipeRaw(stages), "\n")
}

// The capture form of runPipe: same native stage chain, but the LAST
// stage's stdout is read instead of inherited, so `$(cmd | cmd ...)` needs
// no `bash -c` child. Both the read and the Waits must happen before the
// last stage is reaped - a StdoutPipe is not valid after Wait.
func capPipeRaw(stages [][]string) string {
    var cmds []*exec.Cmd
    for _, st := range stages {
        if len(st) == 0 { return "" }
        c := exec.Command(st[0], st[1:]...)
        cmds = append(cmds, c)
    }
    if len(cmds) == 0 { return "" }
    var pipes []io.Closer
    for i := 0; i < len(cmds)-1; i++ {
        p, err := cmds[i].StdoutPipe()
        if err != nil { return "" }
        cmds[i+1].Stdin = p
        pipes = append(pipes, p)
    }
    last, err := cmds[len(cmds)-1].StdoutPipe()
    if err != nil { return "" }
    // After the pipe wiring: unwired Stdin ends follow the process stdin.
    for _, c := range cmds { if c.Stdin == nil { c.Stdin = os.Stdin } }
    for _, c := range cmds {
        if err := c.Start(); err != nil {
            for _, d := range cmds { if d.Process != nil { d.Process.Kill() } }
            for _, d := range cmds { d.Wait() }
            return ""
        }
    }
    data, err := io.ReadAll(last)
    if err != nil { data = nil }
    // Same parent-holds-the-pipe deadlock as runPipe (see above).
    for _, p := range pipes { p.Close() }
    for _, c := range cmds { c.Wait() }
    return string(data)
}
func main() {
    var d string
    var st int64

    fmt.Println("Hello, World!");
    st = 0;
    d = func() string { __mknt1, __mket1 := os.MkdirTemp("", "tmp*"); if __mket1 != nil { return "" }; return __mknt1 }(); // line 6
    st = 0;
    st = 0; // line 7
    if err := os.MkdirAll(fmt.Sprintf("%v/sub1", d), 0o777); err != nil { st = 1 }
    if err := os.MkdirAll(fmt.Sprintf("%v/sub2", d), 0o777); err != nil { st = 1 }
    __ttt2 := time.Now(); // line 8
    st = 0;
    if __e := os.Chtimes(fmt.Sprintf("%v/a.txt", d), __ttt2, __ttt2); __e != nil { if __f, __fe := os.OpenFile(fmt.Sprintf("%v/a.txt", d), os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes(fmt.Sprintf("%v/a.txt", d), __ttt2, __ttt2); __e != nil { st = 1 } } }
    if __e := os.Chtimes(fmt.Sprintf("%v/b.txt", d), __ttt2, __ttt2); __e != nil { if __f, __fe := os.OpenFile(fmt.Sprintf("%v/b.txt", d), os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes(fmt.Sprintf("%v/b.txt", d), __ttt2, __ttt2); __e != nil { st = 1 } } }
    if err := os.Chdir(d); err != nil { st = 1 } else { st = 0 }; // line 9
    st = runPipe([][]string{[]string{"ls", "-1"}, []string{"grep", "-v", "a.txt"}});
    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(capPipe([][]string{[]string{"ls"}, []string{"grep", "-v", "a.txt"}})), " ")));
    st = 0;
    if err := os.Chdir("/"); err != nil { st = 1 } else { st = 0 }; // line 17
    st = 0; // line 18
    if err := os.RemoveAll(d); err != nil { st = 1 }
    _ = st
    os.Exit(int(st));
}