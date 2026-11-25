package main

import (
    "fmt"
    "strings"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    var x int64

    x = 5;
    x += 3;
    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(s2s(x)), " "))); // line 4
    return;
}