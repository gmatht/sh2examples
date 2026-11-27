package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Print("parsed single-quotes in comments OK\n"); // line 4
    st = 0;
    fmt.Println(fmt.Sprintf("done: %v", st)); // line 6
    st = 0;
    return;
}