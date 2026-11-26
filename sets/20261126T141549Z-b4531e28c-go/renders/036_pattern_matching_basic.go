package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "regexp"
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

func b2i(b bool) int64 {
    if b { return 1 }
    return 0
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

func reMatch(pattern, s string) bool {
    ok, _ := regexp.MatchString(pattern, s)
    return ok
}

func main() {
    var st int64

    fmt.Println("== [[ pattern and regex ]]");
    // TODO(unsupported): strHasSuffix
    st = b2i((truthy(0) && (runCmd(exec.Command("echo", "pattern-match")) == 0)));
    st = b2i(!(reMatch(s2s("^file\\.[a-z]+$"), s2s("file.txt"))));
    if st == 0 {
        st = runCmd(exec.Command("echo", "regex-match"));
    }
    _ = st
    return;
}
// 1 construct(s) lowered to TODO markers