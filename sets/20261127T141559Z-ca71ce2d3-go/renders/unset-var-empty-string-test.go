package main

import (
    "fmt"
    "os"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    var never_set2 any

    if (s2s(os.Getenv("never_set")) == s2s("")) { // line 4
        fmt.Println("unset-is-empty");
    } else {
        fmt.Println("unset-not-empty");
    }
    if (!(s2s(never_set2) != "")) {
        fmt.Println("zero-len");
    }
    return;
}