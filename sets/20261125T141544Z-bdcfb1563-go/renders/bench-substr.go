package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    fmt.Println(fmt.Sprintf("sub=%v", s2s("cde")));
    return;
}