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
    var __sh2_tc0 any

    __sh2_tc0 = "foo";
    if (s2i(__sh2_tc0) != 0) {
        fmt.Println("bar");
    }
    return;
}