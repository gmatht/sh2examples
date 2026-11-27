package main

import (
    "fmt"
    "os"
    "strconv"
    "strings"
)

func s2i(x any) int64 {
    s := strings.TrimSpace(fmt.Sprintf("%v", x))
    if n, err := strconv.ParseInt(s, 0, 64); err == nil { return n }
    return 0
}

func cmpN(a, b int64, op string) int64 {
    switch op {
    case "==": if a == b { return 1 }
    case "!=": if a != b { return 1 }
    case "<": if a < b { return 1 }
    case ">": if a > b { return 1 }
    case "<=": if a <= b { return 1 }
    case ">=": if a >= b { return 1 }
    }
    return 0
}

var fArgs []string

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1 { return os.Args[1:] }
    return nil
}

func paramAt(i int) string {
    a := argsList()
    if i >= 1 && i <= len(a) { return a[i-1] }
    return ""
}

func main() {
    var factorial func()
    var i int64
    var n any
    var result int64

    factorial = func() {
        n = paramAt(1);
        result = 1;
        // declare i
        i = 2;
        for (cmpN(i,s2i(n),"<=") != 0) {
            result = (result * i);
            i = i + 1;
        }
        return;
    }
    fmt.Println(fmt.Sprintf("%v", factorial(&[_]string{ "5" })));
    fmt.Println(fmt.Sprintf("%v", factorial(&[_]string{ "6" })));
    return;
}