package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
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
    var d string
    var tee_result string
    var st int64

    fmt.Println("=== Output and Formatting Commands ===");
    st = 0;
    fmt.Println(fmt.Sprintf("Echo result: %v", "Hello from backticks"));
    st = 0;
    d = func() string { __mknt1, __mket1 := os.MkdirTemp("", "tmp*"); if __mket1 != nil { return "" }; return __mknt1 }(); // line 15
    st = 0;
    if err := os.Chdir(d); err != nil { st = 1 } else { st = 0 }; // line 16
    if st != 0 {
        os.Exit(int(1));
    }
    fmt.Println(fmt.Sprintf("Printf result: %v", "Number: 42, String: test"));
    st = 0;
    tee_result = capPipe([][]string{[]string{"echo", "test output"}, []string{"tee", "test_tee.txt"}});
    st = 0;
    fmt.Println(fmt.Sprintf("Tee result: %v", tee_result)); // line 23
    st = 0;
    st = 0; // line 26
    if err := os.Remove("test_tee.txt"); err != nil && !os.IsNotExist(err) { st = 1 }
    fmt.Println("=== Output and Formatting Commands Complete ===");
    st = 0;
    if err := os.Chdir("/"); err != nil { st = 1 } else { st = 0 }; // line 29
    st = 0; // line 30
    if err := os.RemoveAll(d); err != nil { st = 1 }
    _ = st
    os.Exit(int(st));
}