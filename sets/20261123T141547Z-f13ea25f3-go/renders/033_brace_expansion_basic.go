package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Println("== Basic brace expansion ==");
    st = 0;
    fmt.Println("1 2 3 4 5");
    st = 0;
    fmt.Println("a b c");
    st = 0;
    fmt.Println("00 02 04");
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 11
    st = 0;
    return;
}