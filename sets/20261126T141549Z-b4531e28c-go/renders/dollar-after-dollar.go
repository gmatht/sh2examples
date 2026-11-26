package main

import (
    "fmt"
    "os"
    "strings"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

var fArgs []string

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1 { return os.Args[1:] }
    return nil
}

func main() {
    var st int64

    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(s2s(st)), " "))); // line 4
    st = 0;
    fmt.Println(fmt.Sprintf("%v", strings.Join(argsList(), " "))); // line 5
    st = 0;
    fmt.Println(fmt.Sprintf("%v", strings.Join(argsList(), " "))); // line 6
    st = 0;
    return;
}