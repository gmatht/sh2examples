package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    var p any
    var result string

    p = "/home/user/script"; // line 4
    // TODO(unsupported): capture // line 5
    result = s2s(0);
    fmt.Println(fmt.Sprintf("%v", result)); // line 6
    _ = p
    return;
}
// 1 construct(s) lowered to TODO markers