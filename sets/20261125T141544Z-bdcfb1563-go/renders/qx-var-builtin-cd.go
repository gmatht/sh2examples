package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    var script_dir string

    // TODO(unsupported): capture // line 3
    script_dir = s2s(0);
    fmt.Println(fmt.Sprintf("%v", script_dir)); // line 4
    return;
}
// 1 construct(s) lowered to TODO markers