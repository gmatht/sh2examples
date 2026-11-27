package main

import (
    "fmt"
)

func main() {
    var i int64
    var st int64

    i = 0; // line 5
    st = 0;
    i = i + 1;
    st = 0;
    fmt.Println(fmt.Sprintf("post0=%v", st)); // line 7
    st = 0;
    i = i + 1;
    st = 0;
    fmt.Println(fmt.Sprintf("pre=%v", st)); // line 9
    st = 0;
    i = 5; // line 10
    st = 0;
    i = i + 1;
    st = 0;
    fmt.Println(fmt.Sprintf("post5=%v", st)); // line 12
    st = 0;
    _ = i
    return;
}