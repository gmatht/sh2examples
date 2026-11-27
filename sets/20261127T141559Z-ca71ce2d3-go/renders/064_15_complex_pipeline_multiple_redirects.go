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

    st = redirRun("'grep' '-v' '^#' '/etc/passwd' | 'cut' '-d' ':' '-f' '1,3' | 'sort' '-t' ':' '-k' '2' '-n' | ( 'head' '-5' ) >users.txt 2>errors.log", []string{});
    fmt.Println("Pipeline completed");
    st = 0;
    fmt.Println(fmt.Sprintf("done: %v", st)); // line 7
    st = 0;
    return;
}