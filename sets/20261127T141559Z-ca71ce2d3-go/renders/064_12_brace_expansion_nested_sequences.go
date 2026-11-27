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

func main() {
    var st int64

    st = runCmd(exec.Command("mkdir", "-p", "project/src/main/java", "project/src/main/resources", "project/src/test/java", "project/src/test/resources", "project/docs/api", "project/docs/user", "project/build/classes", "project/build/lib"));
    fmt.Println("Project structure created");
    st = 0;
    fmt.Println(fmt.Sprintf("done: %v", st)); // line 7
    st = 0;
    return;
}