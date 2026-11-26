package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    var st int64

    if (!(s2s(st) != "0")) {
        fmt.Println(fmt.Sprintf("%v", (1 + 2)));
        st = 0;
    } else {
        fmt.Println("other");
        st = 0;
    }
    return;
}