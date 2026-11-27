package main

import (
    "fmt"
)

func main() {
    var VAR any
    var VAR2 any

    VAR = "hello"; // line 3
    VAR2 = "world"; // line 4
    fmt.Print("${VAR} more ${VAR2} text\n");
    fmt.Print("VAR=[hello]\n");
    fmt.Print("VAR2=[world]\n");
    _ = VAR
    _ = VAR2
    return;
}