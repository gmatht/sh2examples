package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    var other any
    var var_ any

    if ((s2s(var_) != "") && (!(s2s(other) != ""))) {
        fmt.Println("test");
    }
    return;
}