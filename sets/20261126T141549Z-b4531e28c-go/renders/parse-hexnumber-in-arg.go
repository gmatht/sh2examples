package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Println("0xDEADBEEF"); // line 3
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 5
    st = 0;
    return;
}