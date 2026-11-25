package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Println("hello world\r"); // line 2
    st = 0;
    fmt.Println("goodbye\r"); // line 3
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 5
    st = 0;
    return;
}