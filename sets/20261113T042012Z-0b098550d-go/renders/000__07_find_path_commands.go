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
func main() {
    var d string
    var found_files string
    var st int64

    d = func() string { __mknt1, __mket1 := os.MkdirTemp("", "tmp*"); if __mket1 != nil { return "" }; return __mknt1 }(); // line 6
    st = 0;
    st = 0; // line 7
    if err := os.MkdirAll(fmt.Sprintf("%v/sub", d), 0o777); err != nil { st = 1 }
    __ttt2 := time.Now(); // line 8
    st = 0;
    if __e := os.Chtimes(fmt.Sprintf("%v/a.sh", d), __ttt2, __ttt2); __e != nil { if __f, __fe := os.OpenFile(fmt.Sprintf("%v/a.sh", d), os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes(fmt.Sprintf("%v/a.sh", d), __ttt2, __ttt2); __e != nil { st = 1 } } }
    if __e := os.Chtimes(fmt.Sprintf("%v/b.sh", d), __ttt2, __ttt2); __e != nil { if __f, __fe := os.OpenFile(fmt.Sprintf("%v/b.sh", d), os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes(fmt.Sprintf("%v/b.sh", d), __ttt2, __ttt2); __e != nil { st = 1 } } }
    if __e := os.Chtimes(fmt.Sprintf("%v/sub/c.sh", d), __ttt2, __ttt2); __e != nil { if __f, __fe := os.OpenFile(fmt.Sprintf("%v/sub/c.sh", d), os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes(fmt.Sprintf("%v/sub/c.sh", d), __ttt2, __ttt2); __e != nil { st = 1 } } }
    if __e := os.Chtimes(fmt.Sprintf("%v/data.txt", d), __ttt2, __ttt2); __e != nil { if __f, __fe := os.OpenFile(fmt.Sprintf("%v/data.txt", d), os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes(fmt.Sprintf("%v/data.txt", d), __ttt2, __ttt2); __e != nil { st = 1 } } }
    found_files = func() string { __co3 := os.Stdout; __cf3, __fe3 := os.CreateTemp("", "sh2cap"); if __fe3 != nil { return "" }; os.Stdout = __cf3; if err := os.Chdir(d); err != nil { st = 1 } else { st = 0 };; if st == 0 {;     st = runPipe([][]string{[]string{"find", ".", "-name", "*.sh", "-type", "f"}, []string{"sort"}});; }; __cf3.Close(); os.Stdout = __co3; __cr3, _ := os.ReadFile(__cf3.Name()); os.Remove(__cf3.Name()); return strings.TrimRight(string(__cr3), "\n") }();
    st = 0;
    fmt.Println("Found shell scripts:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", found_files)); // line 11
    st = 0;
    st = 0; // line 12
    if err := os.RemoveAll(d); err != nil { st = 1 }
    _ = st
    os.Exit(int(st));
}