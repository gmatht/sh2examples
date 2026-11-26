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
    var a int64
    var b int64
    var i int64
    var temp int64

    fmt.Println("=== Fibonacci Sequence (first 20 numbers) ===");
    a = 0; // line 9
    b = 1; // line 10
    fmt.Println("Fibonacci numbers:");
    fmt.Print(fmt.Sprintf("%v %v ", a, b)); // line 13
    for _, i = range []int64{s2i("3"), s2i("4"), s2i("5"), s2i("6"), s2i("7"), s2i("8"), s2i("9"), s2i("10"), s2i("11"), s2i("12"), s2i("13"), s2i("14"), s2i("15"), s2i("16"), s2i("17"), s2i("18"), s2i("19"), s2i("20")} {
        temp = (a + b);
        fmt.Print(fmt.Sprintf("%v ", temp));
        a = b;
        b = temp;
    }
    fmt.Println("");
    fmt.Println("Done!");
    _ = i
    return;
}