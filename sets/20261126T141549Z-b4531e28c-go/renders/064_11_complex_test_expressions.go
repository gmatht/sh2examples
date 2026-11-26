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
    if (reMatch(s2s("^[0-9]+$"), s2s(42)) && ((s2s("debug") == s2s("test")) || (s2s("debug") == s2s("debug")))) {
        fmt.Println("Valid input");
    }
    return;
}