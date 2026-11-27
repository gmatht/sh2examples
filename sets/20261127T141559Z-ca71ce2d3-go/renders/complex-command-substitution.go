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

func main() {
    var d string
    var result string
    var st int64

    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 6
    // TODO(unsupported): pipeline // line 7
    st = 0;
    // TODO(unsupported): pipeline // line 8
    st = 0;
    result = s2s(capRun("( 'gzip' '-cdfq' '--' \"$d/f1\" ) 2>/dev/null; ( 'gzip' '-cdfq' '--' \"$d/f2\" ) 2>/dev/null", []string{"d=" + s2s(d), "result=" + s2s(result)})); // line 9
    st = 0;
    fmt.Print(fmt.Sprintf("result=%v\n", result)); // line 13
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 14
    _ = st
    return;
}
// 2 construct(s) lowered to TODO markers