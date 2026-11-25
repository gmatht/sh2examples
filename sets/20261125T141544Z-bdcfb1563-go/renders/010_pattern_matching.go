package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "path/filepath"
    "regexp"
    "strings"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
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

func globMatch(pattern, s string) bool {
    ok, _ := filepath.Match(pattern, s)
    return ok
}

func reMatch(pattern, s string) bool {
    ok, _ := regexp.MatchString(pattern, s)
    return ok
}

func main() {
    var st int64

    fmt.Println("== [[ pattern and regex ]]");
    st = b2i(!(globMatch(s2s("*.txt"), s2s("file.txt"))));
    if st == 0 {
        st = runCmd(exec.Command("echo", "pattern-match"));
    }
    st = b2i(!(reMatch(s2s("^file\\.[a-z]+$"), s2s("file.txt"))));
    if st == 0 {
        st = runCmd(exec.Command("echo", "regex-match"));
    }
    fmt.Println("== extglob ==");
    st = 0;
    st = b2i(!((strings.HasSuffix(s2s("file.js"), ".js") && !globMatch("*.min", strings.TrimSuffix(s2s("file.js"), ".js")))));
    if st == 0 {
        st = runCmd(exec.Command("echo", "f1-ok"));
    }
    st = b2i(!((strings.HasSuffix(s2s("thing.min.js"), ".js") && !globMatch("*.min", strings.TrimSuffix(s2s("thing.min.js"), ".js")))));
    if st != 0 {
        st = runCmd(exec.Command("echo", "f2-filtered"));
    }
    fmt.Println("== nocasematch ==");
    st = 0;
    st = b2i(!(strings.EqualFold(s2s("Foo"), s2s("foo"))));
    if st == 0 {
        st = runCmd(exec.Command("echo", "ci-match"));
    }
    _ = st
    return;
}