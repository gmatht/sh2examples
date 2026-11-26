package main

import (
    "fmt"
    "regexp"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func reMatch(pattern, s string) bool {
    ok, _ := regexp.MatchString(pattern, s)
    return ok
}

func main() {
    if reMatch(s2s("^[a-z]{1,3}$"), s2s("abc")) {
        fmt.Println("match");
    }
    return;
}