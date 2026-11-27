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
    fmt.Println("Testing simple case patterns...");
    if (!(paramAt(1) != "test")) {
        fmt.Println("Matched test");
    } else {
        fmt.Println("Default case");
    }
    return;
}