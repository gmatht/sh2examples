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

    os.WriteFile("lib_tmp.sh", []byte("shared_val=secret\n"), 0644); // line 3
    st = redirRun("source './lib_tmp.sh'", []string{}); // line 4
    fmt.Println(fmt.Sprintf("shared=%v", os.Getenv("shared_val"))); // line 5
    st = 0;
    st = runCmd(exec.Command("rm", "-f", "lib_tmp.sh")); // line 6
    _ = st
    return;
}