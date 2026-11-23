package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "path/filepath"
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

func globArgs(pat string) []string {
    m, err := filepath.Glob(pat)
    if err != nil || m == nil { return []string{pat} }
    return m
}

func main() {
    var d string
    var matched string
    var st int64

    st = redirRun("'echo' '-e' 'line1\\nline2\\nTARGET\\nline4\\nline5' | 'grep' '-A' '2' 'TARGET'", []string{"d=" + s2s(d), "matched=" + s2s(matched)});
    st = redirRun("'echo' '-e' 'line1\\nline2\\nTARGET\\nline4\\nline5' | 'grep' '-B' '2' 'TARGET'", []string{"d=" + s2s(d), "matched=" + s2s(matched)});
    st = redirRun("'echo' '-e' 'line1\\nline2\\nTARGET\\nline4\\nline5' | 'grep' '-C' '1' 'TARGET'", []string{"d=" + s2s(d), "matched=" + s2s(matched)});
    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 12
    st = 0;
    st = runCmd(exec.Command("cd", d)); // line 13
    if st != 0 {
        st = runCmd(exec.Command("exit", "1"));
    }
    os.WriteFile("temp_file1.txt", []byte("pattern in file1\n"), 0644);
    os.WriteFile("temp_file2.txt", []byte("no pattern in file2\n"), 0644);
    os.WriteFile("temp_file3.txt", []byte("pattern in file3\n"), 0644);
    fmt.Println("Recursive search results:");
    st = 0;
    st = runCmd(exec.Command("grep", "-r", "pattern", ".", globArgs("--include=*.txt")...));
    fmt.Println("Result 2..."); // line 21
    st = 0;
    st = runPipe([][]string{[]string{"grep", "-l", "pattern", globArgs("*.txt")...}, []string{"sort"}});
    fmt.Println("Result 3..."); // line 25
    st = 0;
    st = runCmd(exec.Command("grep", "-L", "pattern", globArgs("*.txt")...));
    _ = os.Chdir("/"); // line 29
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 30
    // TODO(unsupported): capture
    matched = s2s(0);
    st = 0;
    fmt.Println(fmt.Sprintf("  grep_exit: %v", st)); // line 33
    st = 0;
    fmt.Println(fmt.Sprintf("  match_count: %v", matched)); // line 34
    st = 0;
    return;
}
// 1 construct(s) lowered to TODO markers