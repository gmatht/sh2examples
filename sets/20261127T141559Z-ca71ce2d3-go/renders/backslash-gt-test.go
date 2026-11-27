package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    if (s2s(5) > s2s(10)) {
        fmt.Println("a > b");
    }
    fmt.Print("a=[5]\n");
    fmt.Print("b=[10]\n");
    return;
}