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

func cmpN(a, b int64, op string) int64 {
    switch op {
    case "==": if a == b { return 1 }
    case "!=": if a != b { return 1 }
    case "<": if a < b { return 1 }
    case ">": if a > b { return 1 }
    case "<=": if a <= b { return 1 }
    case ">=": if a >= b { return 1 }
    }
    return 0
}

func arrIdx(a []any, i int64) any {
    if i >= 0 && int(i) < len(a) { return a[i] }
    return nil
}

func main() {
    var arr []any

    arr = []any{"10", "20", "30"}; // line 4
    fmt.Println(fmt.Sprintf("%v", cmpN(s2i(arrIdx(arr, 1)),2,">"))); // line 5
    return;
}