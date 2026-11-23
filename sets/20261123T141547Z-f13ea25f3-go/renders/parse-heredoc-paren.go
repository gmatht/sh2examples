package main

import (
    "fmt"
)

func main() {
    var IFS any

    fmt.Print("$1\n"); // line 4
    IFS = "\n"; // line 7
    fmt.Print("IFS=[\n]\n");
    _ = IFS
    return;
}