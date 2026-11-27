package main

import (
    "fmt"
)

func main() {
    var i int64
    var j any

    i = 2;
    fmt.Println(fmt.Sprintf("i=%v", i)); // line 5
    t1 := i; // line 6
    i = i + 1;
    i = i + 1;
    j = (t1 + i);
    fmt.Println(fmt.Sprintf("j=%v", j)); // line 7
    return;
}