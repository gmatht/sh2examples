package main

import (
    "fmt"
)

func main() {
    var i int64

    i = 0;
    for (i < 10) {
        fmt.Print(fmt.Sprintf("%v", i));
        i = (i + 1);
    }
    fmt.Println("");
    return;
}