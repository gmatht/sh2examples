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
    var matched string
    var st int64

    st = redirRun("'echo' '-e' 'match1\\nmatch2\\nmatch3\\nmatch4' | 'grep' '-m' '2' 'match'", []string{"matched=" + s2s(matched)});
    st = redirRun("'echo' 'text with pattern in it' | 'grep' '-b' 'pattern'", []string{"matched=" + s2s(matched)});
    os.WriteFile("temp_file.txt", []byte("content\n"), 0644);
    st = runCmd(exec.Command("grep", "-h", "content", "temp_file.txt"));
    st = runCmd(exec.Command("grep", "-H", "content", "temp_file.txt"));
    st = runPipe([][]string{[]string{"grep", "-Z", "-l", "pattern", "temp_file.txt"}, []string{"tr", "\\0", "\\n"}});
    st = redirRun("'echo' 'text with pattern in it' | 'grep' '--color=always' 'pattern'", []string{"matched=" + s2s(matched)});
    if st != 0 {
        st = runCmd(exec.Command("echo", "Color not supported"));
    }
    st = runCmd(exec.Command("grep", "-q", "pattern", "temp_file.txt"));
    if st == 0 {
        st = runCmd(exec.Command("echo", "found"));
    }
    if st != 0 {
        st = runCmd(exec.Command("echo", "not found"));
    }
    st = runCmd(exec.Command("rm", "temp_file.txt")); // line 29
    // TODO(unsupported): capture
    matched = s2s(0);
    st = 0;
    fmt.Println(fmt.Sprintf("  grep_exit: %v", st)); // line 32
    st = 0;
    fmt.Println(fmt.Sprintf("  match_count: %v", matched)); // line 33
    st = 0;
    return;
}
// 1 construct(s) lowered to TODO markers