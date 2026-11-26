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

func truthy(x any) bool {
    switch v := x.(type) {
    case string:
        return v != ""
    case bool:
        return v
    case int64:
        return v != 0
    case nil:
        return false
    }
    return true
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

func capCmd(c *exec.Cmd) string {
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func main() {
    var d string
    var file any
    var st int64

    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 6
    st = runCmd(exec.Command("cd", d)); // line 7
    if st != 0 {
        st = runCmd(exec.Command("exit", "1"));
    }
    st = runCmd(exec.Command("touch", "a.txt", "b.log", "c.dat", "x.yz")); // line 8
    for _, file = range []any{"\x01SH2GLOB\x01*.txt", "\x01SH2GLOB\x01*.log", "\x01SH2GLOB\x01*.dat"} {
        // TODO(unsupported): strHasSuffix
        // TODO(unsupported): strHasSuffix
        if (truthy(0) || truthy(0)) {
            fmt.Println(fmt.Sprintf("Text file: %v", file));
            st = 0;
        // TODO(unsupported): strHasSuffix
        } else if truthy(0) {
            fmt.Println(fmt.Sprintf("Data file: %v", file));
            st = 0;
        } else {
            fmt.Println(fmt.Sprintf("Other file: %v", file));
            st = 0;
        }
    }
    _ = os.Chdir("/"); // line 16
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 17
    _ = st
    return;
}
// 3 construct(s) lowered to TODO markers