package main

import (
    "fmt"
)

func main() {
    var count int64

    count = 3;
    for (!(count == 0)) {
        fmt.Println(fmt.Sprintf("count=%v", count));
        count = (count - 1);
    }
    return;
}