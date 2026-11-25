package main

import (
    "fmt"
    "os"
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
    if globMatch(s2s("?*"), s2s(os.Getenv("var"))) {
        fmt.Println("has chars");
    }
    return;
}