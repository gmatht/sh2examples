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
    var shell any
    var uid any
    var st int64

    st = redirRun("( while 'read' '-r' 'user' 'pass' 'uid' 'gid' 'info' 'home' 'shell'; do if $(($(($uid)) > $((1000)))) && $(($shell != /bin/false)); then if [ \"$home\"=~^/home/ ]; then echo User: $user (UID: $uid) - $home; fi; fi; done ) 0</etc/passwd", []string{"shell=" + s2s(shell), "uid=" + s2s(uid)});
    _ = shell
    _ = uid
    _ = st
    return;
}