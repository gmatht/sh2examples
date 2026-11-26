package main

import (
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

    st = redirRun("( 'cat' ) 0<<'EOF'\n# This is a comment\n$(echo \"Command substitution\")\n${var:-default}\n$(( 1 + 2 * 3 ))\n\nEOF | 'grep' '-v' '^#' | 'sed' 's/^/  /'", []string{});
    _ = st
    return;
}