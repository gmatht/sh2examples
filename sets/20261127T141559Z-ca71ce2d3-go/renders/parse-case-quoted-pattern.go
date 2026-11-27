package main

import (
    "fmt"
    "os"
    "path/filepath"
)

func globMatch(pattern, s string) bool {
    ok, _ := filepath.Match(pattern, s)
    return ok
}

func main() {
    switch { // line 5
        case globMatch("*'\n'*", os.Getenv("i")) || globMatch("*'&'*", os.Getenv("i")) || globMatch("*'\\'*", os.Getenv("i")) || globMatch("*'|'*", os.Getenv("i")):
            fmt.Println("match");
        default:
            _ = 0;
    }
    return;
}