package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    var __sh2_tc0 any

    __sh2_tc0 = "hello";
    if (s2s(__sh2_tc0) == s2s("hello")) {
        fmt.Println("match");
    }
    return;
}