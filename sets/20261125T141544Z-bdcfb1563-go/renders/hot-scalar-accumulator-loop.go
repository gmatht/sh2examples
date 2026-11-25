package main

import (
    "fmt"
)

func main() {
    var i int64
    var s int64

    s = 0; // line 4
    i = 0;
    for (i < 100) {
        s = (s + i);
        i = (i + 1);
    }
    fmt.Println(fmt.Sprintf("sum=%v", s)); // line 10
    return;
}