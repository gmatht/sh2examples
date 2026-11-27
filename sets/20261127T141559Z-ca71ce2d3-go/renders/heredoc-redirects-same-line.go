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

func redirRun(script string, env []string) int64 {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    return runCmd(c)
}

func main() {
    var tmpf string
    var st int64

    tmpf = s2s(capCmd(exec.Command("mktemp", "/tmp/heredoc_redirect_same_line.XXXXXX"))); // line 3
    st = redirRun("( ( echo -n hello ) ) 0<<'EOF'\nbody\n\nEOF 2>&1 1>${tmpf}", []string{"tmpf=" + s2s(tmpf)}); // line 4
    fmt.Print(fmt.Sprintf("heredoc+redirects output=[%v]\n", capRun("( 'cat' \"$tmpf\" ) 2>/dev/null || 'echo' '(empty)'", []string{"tmpf=" + s2s(tmpf)}))); // line 7
    st = 0;
    st = runCmd(exec.Command("rm", "-f", tmpf)); // line 8
    _ = st
    return;
}