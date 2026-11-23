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

func arrIdx(a []any, i int64) any {
    if i >= 0 && int(i) < len(a) { return a[i] }
    return nil
}

func main() {
    var arr []any
    var i int64

    arr = []any{"10", "20", "30"}; // line 3
    i = 1; // line 4
    fmt.Println(fmt.Sprintf("%v", (s2i(arrIdx(arr, i)) * 2))); // line 5
    fmt.Println(fmt.Sprintf("%v", (s2i(arrIdx(arr, 0)) + s2i(arrIdx(arr, 2))))); // line 6
    fmt.Println(fmt.Sprintf("%v", ((s2i(arrIdx(arr, 1)) + s2i(arrIdx(arr, 2))) & 50))); // line 7
    return;
}