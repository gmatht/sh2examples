package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
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

func capCmd(c *exec.Cmd) string {
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func capRun(script string, env []string) string {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
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
    var d string
    var st int64

    fmt.Println("Hello, World!");
    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 6
    st = runCmd(exec.Command("mkdir", "-p", fmt.Sprintf("%v/sub1", d), fmt.Sprintf("%v/sub2", d))); // line 7
    st = runCmd(exec.Command("touch", fmt.Sprintf("%v/a.txt", d), fmt.Sprintf("%v/b.txt", d))); // line 8
    _ = os.Chdir(d); // line 9
    st = 0;
    st = runPipe([][]string{[]string{"ls", "-1"}, []string{"grep", "-v", "a.txt"}});
    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(capRun("'ls' | 'grep' '-v' 'a.txt'", []string{"d=" + s2s(d)})), " ")));
    st = 0;
    _ = os.Chdir("/"); // line 17
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 18
    _ = st
    return;
}