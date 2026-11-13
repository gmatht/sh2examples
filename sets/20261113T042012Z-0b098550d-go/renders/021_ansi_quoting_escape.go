package main

import (
    "fmt"
    "os"
)

func main() {
    var st int64

    fmt.Println("== Escape sequences ==");
    st = 0;
    fmt.Println("bell\x07"); // line 7
    st = 0;
    fmt.Println("backspace\x08"); // line 8
    st = 0;
    fmt.Println("formfeed\x0c"); // line 9
    st = 0;
    fmt.Println("newline\n"); // line 10
    st = 0;
    fmt.Println("carriage\rreturn"); // line 11
    st = 0;
    fmt.Println("tab\tseparated"); // line 12
    st = 0;
    fmt.Println("vertical\x0btab"); // line 13
    st = 0;
    _ = st
    os.Exit(int(st));
}