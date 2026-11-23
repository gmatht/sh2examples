package main

import (
    "fmt"
    "strconv"
    "strings"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func s2i(x any) int64 {
    s := strings.TrimSpace(fmt.Sprintf("%v", x))
    if n, err := strconv.ParseInt(s, 0, 64); err == nil { return n }
    return 0
}

func main() {
    var i int64
    var j int64

    for _, i = range []int64{s2i("1"), s2i("2"), s2i("3"), s2i("4"), s2i("5")} {
        fmt.Println(fmt.Sprintf("Number: %v", i));
    }
    for _, i = range []int64{s2i("1"), s2i("2"), s2i("3")} {
        j = (j + 1);
    }
    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(s2s(j)), " "))); // line 8
    i = 0;
    for (i < 10) {
        fmt.Println(fmt.Sprintf("Counter: %v", i));
        i = (i + 1);
    }
    return;
}