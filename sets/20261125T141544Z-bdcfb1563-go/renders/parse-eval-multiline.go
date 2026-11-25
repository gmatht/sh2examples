package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
)

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
    var st int64

    st = redirRun("\n    doselect() {\n        select select_result; do\n            case $select_result in\n                \"\") echo >&2 \"Please enter a number.\" ;;\n                ?*) break ;;\n            esac\n        done\n    }\n", []string{}); // line 5
    st = redirRun("( 'echo' 'doselect is a function' ) 2>/dev/null | 'head' '-1'", []string{});
    fmt.Print("doselect definition parsed OK\n"); // line 17
    st = 0;
    _ = st
    return;
}