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

func redirRun(script string, env []string) int64 {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    return runCmd(c)
}

func main() {
    var __ps_tmp0 string
    var counter int64
    var input_file string
    var max_lines int64
    var st int64

    input_file = s2s(capCmd(exec.Command("mktemp"))); // line 4
    max_lines = 3; // line 5
    counter = 0; // line 6
    st = redirRun("( echo -n # comment\nhas $(sub) here\nplain line\n${param} line\n ) 1>${input_file}", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "counter=" + s2s(counter), "input_file=" + s2s(input_file), "max_lines=" + s2s(max_lines)}); // line 7
    // TODO(unsupported): capture
    __ps_tmp0 = s2s(0);
    st = 0;
    // TODO(unsupported): Redirect
    // TODO(unsupported): Redirect
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp0", None)], capture: None, redirects: [], env: [] }
    st = runCmd(exec.Command("rm", "-f", input_file)); // line 27
    _ = __ps_tmp0
    _ = counter
    _ = max_lines
    _ = st
    return;
}
// 4 construct(s) lowered to TODO markers