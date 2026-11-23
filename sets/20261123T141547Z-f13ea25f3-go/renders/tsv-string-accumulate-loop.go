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
    var tsv string

    tsv = "";
    for _, i = range []int64{s2i("1"), s2i("2"), s2i("3"), s2i("4"), s2i("5")} { // line 4
        tsv = fmt.Sprintf("%v%v,", tsv, i);
    }
    fmt.Println(fmt.Sprintf("%v", tsv)); // line 7
    return;
}