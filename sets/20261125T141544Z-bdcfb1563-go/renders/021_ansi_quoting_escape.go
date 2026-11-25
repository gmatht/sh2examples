package main

import (
    "fmt"
)

func main() {
    fmt.Println("== Escape sequences ==");
    fmt.Println("bell\x07"); // line 7
    fmt.Println("backspace\x08"); // line 8
    fmt.Println("formfeed\x0c"); // line 9
    fmt.Println("newline\n"); // line 10
    fmt.Println("carriage\rreturn"); // line 11
    fmt.Println("tab\tseparated"); // line 12
    fmt.Println("vertical\x0btab"); // line 13
    return;
}