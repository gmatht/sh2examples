package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
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
    var VAR any
    var my_func func()
    var st int64

    // TODO(unsupported): assign
    st = b2i(((s2s(VAR) != "") || truthy(0)));
    my_func = func() {
        return;
    }
    fmt.Println(fmt.Sprintf("%v", my_func(&[_]string{  })));
    st = 0;
    _ = st
    return;
}
// 1 construct(s) lowered to TODO markers