package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    var output string

    // TODO(unsupported): capture
    output = s2s(0);
    fmt.Println(fmt.Sprintf("%v", output)); // line 5
    return;
}
// 1 construct(s) lowered to TODO markers