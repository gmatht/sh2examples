package main

import (
    "fmt"
    "os"
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

func main() {
    if (!(paramAt(1) != "a")) {
        if (!(paramAt(2) != "x")) {
            fmt.Println("x");
        }
    } else if (!(paramAt(1) != "b")) {
        fmt.Println("b");
    }
    return;
}