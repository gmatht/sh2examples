package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
    "time"
)


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
    var formatted_date string
    var yes_result string
    var st int64

    fmt.Println("=== System Utilities ===");
    st = 0;
    formatted_date = time.Now().Format("2006-01-02"); // line 11
    st = 0;
    fmt.Println(fmt.Sprintf("Formatted date: %v", formatted_date)); // line 13
    st = 0;
    yes_result = capPipe([][]string{[]string{"yes", "Hello"}, []string{"head", "-3"}});
    st = 0;
    fmt.Println("Yes command result:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", yes_result)); // line 35
    st = 0;
    _ = st
    os.Exit(int(st));
}