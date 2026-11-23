package main

import (
    "fmt"
)

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

func main() {
    // TODO(unsupported): strHasSuffix
    if truthy(0) {
        fmt.Println("case=match");
    } else {
        fmt.Println("case=no");
    }
    // TODO(unsupported): strHasSuffix
    if truthy(0) {
        fmt.Println("case2=match");
    } else {
        fmt.Println("case2=no");
    }
    return;
}
// 2 construct(s) lowered to TODO markers