package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Println("it's works"); // line 2
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 4
    st = 0;
    return;
}