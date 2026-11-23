package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func b2i(b bool) int64 {
    if b { return 1 }
    return 0
}

func main() {
    var st int64

    st = b2i(!((s2s("abc") == s2s("abc")))); // line 3
    fmt.Println(fmt.Sprintf("eq=%v", st)); // line 3
    st = 0;
    st = b2i(!((s2s("abc") != s2s("xyz")))); // line 4
    fmt.Println(fmt.Sprintf("ne=%v", st)); // line 4
    st = 0;
    st = b2i(!((5 < 10))); // line 5
    fmt.Println(fmt.Sprintf("lt=%v", st)); // line 5
    st = 0;
    return;
}