package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    if (s2s("a") == s2s("a")) { // line 5
        fmt.Println("eq");
    }
    if (s2s("a") != s2s("a")) { // line 6
        fmt.Println("bad");
    } else {
        fmt.Println("ne");
    }
    if (s2s("a") == s2s("b")) { // line 7
        fmt.Println("bad");
    } else {
        fmt.Println("not-equal");
    }
    if (s2s("hello") == s2s("hello")) {
        fmt.Println("var-eq");
    }
    if (s2s("hello") != s2s("other")) {
        fmt.Println("var-ne");
    }
    if (s2s(10) == s2s(10)) { // line 13
        fmt.Println("num-eq");
    }
    if (s2s(10) == s2s(10)) { // line 14
        fmt.Println("bad");
    } else {
        fmt.Println("num-string-distinct");
    }
    return;
}