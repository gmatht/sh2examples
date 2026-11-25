package main

import (
    "fmt"
    "strings"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    var f func()
    var i int64

    f = func() {
        return;
    }
    i = 0;
    for (i < 5) {
        fmt.Println(fmt.Sprintf("%v", f(&[_]string{ s2s(strings.Join(strings.Fields(s2s(i)), " ")) })));
        i = (i + 1);
    }
    return;
}