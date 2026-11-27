package main

import (
    "fmt"
    "os"
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
    if reMatch(s2s("DEBUG:"), s2s(os.Getenv("line"))) {
        fmt.Println("match");
    }
    return;
}