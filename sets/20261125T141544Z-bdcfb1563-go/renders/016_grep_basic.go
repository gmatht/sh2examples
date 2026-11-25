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
    var matched string
    var st int64

    st = runCmd(exec.Command("grep", "pattern", "/dev/null"));
    if st != 0 {
        st = runCmd(exec.Command("echo", "No matches found"));
    }
    st = redirRun("'echo' 'HELLO world' | 'grep' '-i' 'hello'", []string{"matched=" + s2s(matched)});
    st = redirRun("'echo' '-e' 'line1\\nline2\\nline3' | 'grep' '-v' 'line2'", []string{"matched=" + s2s(matched)});
    st = redirRun("'echo' '-e' 'first\\nsecond\\nthird' | 'grep' '-n' 'second'", []string{"matched=" + s2s(matched)});
    st = redirRun("'echo' '-e' 'match\\nno match\\nmatch again' | 'grep' '-c' 'match'", []string{"matched=" + s2s(matched)});
    st = redirRun("'echo' 'text with pattern123 in it' | 'grep' '-o' 'pattern[0-9]\\+'", []string{"matched=" + s2s(matched)});
    // TODO(unsupported): capture
    matched = s2s(0);
    st = 0;
    fmt.Println(fmt.Sprintf("  grep_exit: %v", st)); // line 25
    st = 0;
    fmt.Println(fmt.Sprintf("  match_count: %v", matched)); // line 26
    st = 0;
    return;
}
// 1 construct(s) lowered to TODO markers