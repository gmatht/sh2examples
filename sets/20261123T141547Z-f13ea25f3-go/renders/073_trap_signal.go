package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Println("Trap set");
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 6
    st = 0;
    return;
}