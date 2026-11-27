package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    if (s2s("foo") == s2s("@(foo|bar)")) {
        fmt.Println("matched");
    }
    fmt.Print("parsed OK\n");
    return;
}