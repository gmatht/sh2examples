package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Println("== Advanced brace expansion ==");
    st = 0;
    fmt.Println("a1 a2 a3 b1 b2 b3 c1 c2 c3");
    st = 0;
    fmt.Println("1 3 5 7 9");
    st = 0;
    fmt.Println("a d g j m p s v y");
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 11
    st = 0;
    return;
}