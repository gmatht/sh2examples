package main

import (
    "fmt"
)

var fArgs []string

func main() {
    var f func()
    var st int64

    f = func() {
        if true {
            st = 0;
            fmt.Print("hello\n");
            st = 0;
        }
    }
    fArgs = []string{}; f(); fArgs = nil; // line 10
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 12
    st = 0;
    return;
}