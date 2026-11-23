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
    var i int64

    i = (i + 1);
    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(s2s(i)), " "))); // line 4
    return;
}