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

func truthy(x any) bool {
    switch v := x.(type) {
    case string:
        return v != ""
    case bool:
        return v
    case int64:
        return v != 0
    case nil:
        return false
    }
    return true
}

func b2i(b bool) int64 {
    if b { return 1 }
    return 0
}

func main() {
    var i int64
    var st int64

    for _, i = range []int64{s2i("1"), s2i("2"), s2i("3")} {
        // TODO(unsupported): continue
        st = b2i(((!(i != 2)) || truthy(0)));
        fmt.Println(fmt.Sprintf("%v", i));
        st = 0;
    }
    _ = st
    return;
}
// 1 construct(s) lowered to TODO markers