package main

import (
    "fmt"
    "strconv"
    "strings"
)

func s2i(x any) int64 {
    s := strings.TrimSpace(fmt.Sprintf("%v", x))
    if n, err := strconv.ParseInt(s, 0, 64); err == nil { return n }
    return 0
}

func main() {
    var i int64

    fmt.Println("1 2 3 4 5");
    fmt.Println("file.txt file.bak");
    for _, i = range []int64{s2i("1"), s2i("2"), s2i("3")} {
        fmt.Println(fmt.Sprintf("%v", i));
    }
    return;
}