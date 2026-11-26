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
    var var_ any
    var st int64

    var_ = "hello"; // line 3
    st = redirRun("( 'cat' ) 0<<'EOF'\ncontent\n\nEOF", []string{"var=" + s2s(var_)}); // line 4
    st = redirRun("$var", []string{"var=" + s2s(var_)}); // line 9
    fmt.Print(fmt.Sprintf("var after eval=[%v]\n", var_)); // line 10
    st = 0;
    _ = st
    return;
}