package main

import (
    "fmt"
    "strings"
)

func main() {
    var y string

    y = fmt.Sprintf("%vworld", "hello");
    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(y), " "))); // line 5
    fmt.Print("x=[hello]\n");
    return;
}