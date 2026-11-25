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

var fArgs []string

func main() {
    var inner_func func()
    var inner_var any
    var outer_func func()
    var outer_var any
    var result int64

    outer_func = func() { // line 4
        outer_var = "outer";
        inner_func = func() {
            inner_var = "inner";
            fmt.Println(fmt.Sprintf("Outer: %v, Inner: %v", outer_var, inner_var));
            result = (s2i(outer_var) + s2i(inner_var));
            fmt.Println(fmt.Sprintf("Result: %v", result));
        }
        fArgs = []string{}; inner_func(); fArgs = nil;
    }
    fArgs = []string{}; outer_func(); fArgs = nil; // line 20
    return;
}