package main

import (
    "fmt"
    "strconv"
    "strings"
)

func s2i(x any) int64 {
    s := strings.TrimSpace(fmt.Sprintf("%v", x))
    if n, err := strconv.ParseInt(s, 0, 64); err == nil { return n }
    return 0
}

func main() {
    var i int64
    var total int64

    total = 0; // line 4
    for _, i = range []int64{s2i("1"), s2i("2"), s2i("3"), s2i("4"), s2i("5")} { // line 5
        total = (total + i);
    }
    fmt.Println(fmt.Sprintf("total=%v", total)); // line 8
    return;
}