package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    var var_ any

    if (!(s2s(var_) != "true")) {
        fmt.Println("it's true");
    }
    return;
}