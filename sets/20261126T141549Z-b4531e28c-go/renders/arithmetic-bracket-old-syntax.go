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
    var j int64

    i = 0; // line 3
    i = (i + 1); // line 4
    j = (j + 1); // line 5
    fmt.Println(fmt.Sprintf("%v %v", strings.Join(strings.Fields(s2s(i)), " "), strings.Join(strings.Fields(s2s(j)), " "))); // line 6
    return;
}