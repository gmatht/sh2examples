package main

import (
    "fmt"
    "os"
)

func main() {
    var st int64

    fmt.Println("Hello, World!");
    st = 0;
    fmt.Println("Single quoted"); // line 2
    st = 0;
    fmt.Println("String with \"escaped\" quotes");
    st = 0;
    fmt.Println("String with 'single' quotes");
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 6
    st = 0;
    os.Exit(int(st));
}