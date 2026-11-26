package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Println("0x1234"); // line 2
    st = 0;
    fmt.Println(fmt.Sprintf("hex_val: %v", 255)); // line 3
    st = 0;
    fmt.Println(fmt.Sprintf("done: %v", st)); // line 4
    st = 0;
    return;
}