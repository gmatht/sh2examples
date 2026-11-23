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
    switch { // line 4
        case globMatch("[a-z]*", paramAt(1)):
            fmt.Println("Lowercase start");
        case globMatch("[A-Z]*", paramAt(1)):
            fmt.Println("Uppercase start");
        case globMatch("[0-9]*", paramAt(1)):
            fmt.Println("Number start");
        case globMatch("?", paramAt(1)):
            fmt.Println("Single character");
        case globMatch("*", paramAt(1)):
            fmt.Println("Something else");
        default:
            _ = 0;
    }
    return;
}