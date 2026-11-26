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

func main() {
    if (int64(len(argsList())) < 2) {
        fmt.Println("One");
        fmt.Println("Two");
    }
    return;
}