package main

import (
    "fmt"
)

func main() {
    var __n int64
    var i int64

    i = 1; // line 3
    __n = 0;
    for (__n < 1000) {
        i = (i + 1);
        __n = (__n + 1);
    }
    fmt.Println(fmt.Sprintf("count=%v", i)); // line 9
    return;
}