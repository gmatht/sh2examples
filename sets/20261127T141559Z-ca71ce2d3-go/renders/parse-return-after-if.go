package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Print("parsed OK\n");
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 10
    st = 0;
    return;
}