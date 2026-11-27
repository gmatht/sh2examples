package main

import (
    "fmt"
)

func main() {
    fmt.Println("== ANSI-C quoting ==");
    fmt.Println("line1\nline2\tTabbed"); // line 9
    fmt.Println("== Escape sequences ==");
    fmt.Println("bell\x07"); // line 12
    fmt.Println("backspace\x08"); // line 13
    fmt.Println("formfeed\x0c"); // line 14
    fmt.Println("newline\n"); // line 15
    fmt.Println("carriage\rreturn"); // line 16
    fmt.Println("tab\tseparated"); // line 17
    fmt.Println("vertical\x0btab"); // line 18
    fmt.Println("== Unicode and hex ==");
    fmt.Println("Hello"); // line 21
    fmt.Println("Hello"); // line 22
    fmt.Println("== Practical examples ==");
    fmt.Print("Name       Age        City\n");
    fmt.Print("John       25         NYC\n");
    fmt.Print("Jane       30         LA\n");
    return;
}