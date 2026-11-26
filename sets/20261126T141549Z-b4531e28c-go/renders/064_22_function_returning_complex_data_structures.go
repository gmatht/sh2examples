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

var fArgs []string

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
    var get_system_info func()
    var info []any
    var info__arch__ any
    var info__hostname__ any
    var info__os__ any
    var info__user__ any
    var key any
    var st int64

    get_system_info = func() { // line 4
        // declare info
        info__os__ = capCmd(exec.Command("uname", "-s"));
        info__arch__ = capCmd(exec.Command("uname", "-m"));
        info__hostname__ = capCmd(exec.Command("hostname"));
        info__user__ = os.Getenv("USER");
        st = redirRun("for key in ${!infoslice@}; do 'echo' \"info[$key]=${info[$key]}\"; done | 'sort'", []string{"get_system_info=" + s2s(get_system_info), "key=" + s2s(key)});
    }
    fArgs = []string{}; get_system_info(); fArgs = nil; // line 16
    _ = info
    _ = info__arch__
    _ = info__hostname__
    _ = info__os__
    _ = info__user__
    _ = key
    _ = st
    return;
}