package main

import (
    "fmt"
    "path/filepath"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func globMatch(pattern, s string) bool {
    ok, _ := filepath.Match(pattern, s)
    return ok
}

func main() {
    if globMatch(s2s("*--view*"), s2s("*")) {
        fmt.Println("view mode");
    }
    return;
}