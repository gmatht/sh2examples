package main

import (
    "fmt"
    "strconv"
    "strings"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func s2i(x any) int64 {
    s := strings.TrimSpace(fmt.Sprintf("%v", x))
    if n, err := strconv.ParseInt(s, 0, 64); err == nil { return n }
    return 0
}

func main() {
    var i any
    var j any

    j = "1"; // line 1
    for _, i = range []any{"1", "2", "3", "4", "5"} {
        j = (s2i(j) * s2i(i));
    }
    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(s2s(j)), " "))); // line 6
    return;
}