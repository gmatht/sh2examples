package main

import (
    "fmt"
    "os"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    if (s2s(os.Getenv("ZSH_VERSION-")) != "") {
        fmt.Println("zsh");
    } else if (s2s(os.Getenv("BASH_VERSION-")) != "") {
        fmt.Println("bash");
    }
    return;
}