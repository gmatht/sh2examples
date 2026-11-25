package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Println("1 2 3 4 5");
    st = 0;
    fmt.Println("a b c d e");
    st = 0;
    fmt.Println("1..3 7..9");
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 8
    st = 0;
    return;
}