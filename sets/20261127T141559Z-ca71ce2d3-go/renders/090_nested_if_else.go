package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    if (s2s("hello") == s2s("hello")) {
        fmt.Println("greeting");
    } else if (s2s("hello") == s2s("bye")) {
        fmt.Println("farewell");
    } else {
        fmt.Println("unknown");
    }
    return;
}