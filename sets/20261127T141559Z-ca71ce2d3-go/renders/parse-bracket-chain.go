package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    var VAR1 any
    var VAR2 any

    if ((!(s2s(VAR1) != "")) && (!(s2s(VAR2) != ""))) {
        fmt.Println("Both empty");
    }
    return;
}