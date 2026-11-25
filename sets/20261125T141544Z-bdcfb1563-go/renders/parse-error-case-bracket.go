package main

import (
    "fmt"
    "os"
    "path/filepath"
)

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

func globMatch(pattern, s string) bool {
    ok, _ := filepath.Match(pattern, s)
    return ok
}

func main() {
    switch { // line 3
        case globMatch("-[CDFISUWXx]", paramAt(1)):
            fmt.Println("option");
        default:
            _ = 0;
    }
    return;
}