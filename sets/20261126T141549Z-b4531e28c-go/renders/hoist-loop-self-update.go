package main

import (
    "fmt"
)

func main() {
    var i int64

    i = 0;
    for (i < 3) {
        fmt.Println(fmt.Sprintf("i=%v", i));
        i = (i + 1);
    }
    return;
}