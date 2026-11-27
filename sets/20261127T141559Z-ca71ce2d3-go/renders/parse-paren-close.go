package main

import (
    "fmt"
)

func main() {
    var c int64

    c = 13;
    fmt.Print(fmt.Sprintf("a=[%v]\n", 9));
    fmt.Print(fmt.Sprintf("b=[%v]\n", 4)); // line 6
    fmt.Print(fmt.Sprintf("c=[%v]\n", c)); // line 7
    fmt.Println(fmt.Sprintf("%v", (13 + c))); // line 8
    return;
}