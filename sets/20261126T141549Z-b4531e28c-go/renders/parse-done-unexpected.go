package main

import (
    "fmt"
)

func main() {
    for true {
        fmt.Println("loop");
        break;
    }
    fmt.Println(fmt.Sprintf("status: %v", "ok"));
    return;
}