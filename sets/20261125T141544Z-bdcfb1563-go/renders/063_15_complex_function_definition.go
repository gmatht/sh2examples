package main

import (
    "fmt"
    "os"
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
    var complex_func func()
    var result any
    var x any
    var y any

    complex_func = func() { // line 4
        x = paramAt(1);
        y = paramAt(2);
        result = s2s((s2i(x) + s2i(y)));
        fmt.Println(fmt.Sprintf("Sum: %v", result));
        fmt.Println(fmt.Sprintf("Args: %v %v", x, y));
    }
    fArgs = []string{"3", "7"}; complex_func(); fArgs = nil; // line 13
    fArgs = []string{"10", "20"}; complex_func(); fArgs = nil; // line 14
    return;
}