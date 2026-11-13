package main

import (
    "fmt"
    "os"
)

func main() {
    var st int64

    fmt.Println("== ANSI-C quoting ==");
    st = 0;
    fmt.Println("line1\nline2\tTabbed"); // line 7
    st = 0;
    _ = st
    os.Exit(int(st));
}