package main

import (
    "fmt"
    "os"
)

func main() {
    var st int64

    fmt.Println("== Unicode and hex ==");
    st = 0;
    fmt.Println("Hello"); // line 7
    st = 0;
    fmt.Println("Hello"); // line 8
    st = 0;
    _ = st
    os.Exit(int(st));
}