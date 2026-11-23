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
    var var_ any

    fmt.Println(fmt.Sprintf("%v", (s2i(var_) + 1))); // line 4
    return;
}