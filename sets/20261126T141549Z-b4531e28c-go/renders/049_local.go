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
    var a any

    a = "1"; // line 1
    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(s2s(a)), " "))); // line 2
        t1 := a; // line 3
    a = "2";
    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(s2s(a)), " ")));
    a = t1;
    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(s2s(a)), " "))); // line 4
    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(s2s(a)), " "))); // line 5
    return;
}