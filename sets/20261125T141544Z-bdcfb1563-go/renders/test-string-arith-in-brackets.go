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
    var a []any

    a = []any{"10", "20", "30", "40"}; // line 3
    if (1 < (s2i(arrIdx(a, 1)) + s2i(arrIdx(a, 2)))) {
        fmt.Println("arith-test-ok");
    } else {
        fmt.Println("arith-test-bad");
    }
    fmt.Println("done"); // line 6
    return;
}