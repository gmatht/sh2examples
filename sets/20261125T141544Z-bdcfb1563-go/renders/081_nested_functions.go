package main

import (
    "fmt"
)

var fArgs []string

func main() {
    var inner func()
    var msg any
    var outer func()

    outer = func() { // line 3
        msg = "hello";
        inner = func() {
            fmt.Println(fmt.Sprintf("%v", msg));
        }
        fArgs = []string{}; inner(); fArgs = nil;
    }
    fArgs = []string{}; outer(); fArgs = nil; // line 10
    return;
}