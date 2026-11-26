package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Print("a [Y/n] b\n");
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 6
    st = 0;
    return;
}