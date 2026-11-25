package main

import (
    "fmt"
)

func main() {
    var i int64
    var j any

    i = 2;
    t1 := i; // line 5
    i = i + 1;
    i = i + 1;
    j = (t1 + i);
    fmt.Println(fmt.Sprintf("i=%v, j=%v", i, j)); // line 6
    return;
}