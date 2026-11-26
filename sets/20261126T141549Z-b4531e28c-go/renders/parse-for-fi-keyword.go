package main

import (
    "fmt"
)

func main() {
    var lang any

    for _, lang = range []any{"cs", "de", "es", "fi", "fr"} { // line 2
        fmt.Println(fmt.Sprintf("%v", lang));
    }
    return;
}