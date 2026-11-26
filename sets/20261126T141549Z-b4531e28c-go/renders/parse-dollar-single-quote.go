package main

import (
    "fmt"
)

func main() {
    var IFS any

    IFS = "\n\t"; // line 3
    fmt.Print("IFS value (hex)="); // line 4
    fmt.Print("$'\\n\\t'");
    fmt.Print("\n"); // line 4
    _ = IFS
    return;
}