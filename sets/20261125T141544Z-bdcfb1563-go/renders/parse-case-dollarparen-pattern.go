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

func capCmd(c *exec.Cmd) string {
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func main() {
    var __sh2_casesubj_0 any

    __sh2_casesubj_0 = capCmd(exec.Command("__docker_to_extglob", os.Getenv("subcommands")));
    if (!(s2s(__sh2_casesubj_0) != "foo")) {
        fmt.Println("bar");
    }
    return;
}