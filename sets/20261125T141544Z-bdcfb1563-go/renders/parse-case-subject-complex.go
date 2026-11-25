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

var fArgs []string

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1 { return os.Args[1:] }
    return nil
}

func paramAt(i int) string {
    a := argsList()
    if i >= 1 && i <= len(a) { return a[i-1] }
    return ""
}

func capCmd(c *exec.Cmd) string {
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func main() {
    var __sh2_casesubj_0 any

    __sh2_casesubj_0 = fmt.Sprintf("%v-%v", paramAt(1), capCmd(exec.Command("uname", "-s")));
    if (!(s2s(__sh2_casesubj_0) != "regex-Linux")) {
        fmt.Println("Linux");
    // TODO(unsupported): strHasPrefix
    } else if truthy(0) {
        fmt.Println(fmt.Sprintf("Other: %v", capCmd(exec.Command("uname", "-s"))));
    }
    return;
}
// 1 construct(s) lowered to TODO markers