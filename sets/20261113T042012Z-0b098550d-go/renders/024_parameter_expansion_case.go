package main

import (
    "fmt"
    "os"
)

func main() {
    var st int64

    fmt.Println("== Case modification in parameter expansion ==");
    st = 0;
    fmt.Println("WORLD");
    st = 0;
    fmt.Println("world");
    st = 0;
    fmt.Println("World");
    st = 0;
    _ = st
    os.Exit(int(st));
}