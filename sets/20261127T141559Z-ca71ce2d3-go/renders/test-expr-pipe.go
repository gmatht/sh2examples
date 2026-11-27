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
    if reMatch(s2s("bullseye|bookworm|jammy"), s2s(os.Getenv("release"))) {
        fmt.Println("match");
    }
    return;
}