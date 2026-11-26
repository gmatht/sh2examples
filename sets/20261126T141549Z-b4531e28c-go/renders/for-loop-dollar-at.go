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

var fArgs []string

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1 { return os.Args[1:] }
    return nil
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
    var X_SET int64
    var arg any

    X_SET = 0; // line 4
    t1 := argsList(); // line 5
    for _, t2 := range t1 {
        arg = t2;
        if (redirRun("'echo' \"$arg\" | 'grep' '-q' '-e' '^-[a-zA-Z0-9]*x'", []string{"X_SET=" + s2s(X_SET), "arg=" + s2s(arg)}) == 0) {
            X_SET = 1;
        }
    }
    fmt.Println(fmt.Sprintf("%v", X_SET)); // line 10
    _ = arg
    return;
}