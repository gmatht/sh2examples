package main

import (
    "fmt"
)

func main() {
    var exitScript func()
    var st int64

    exitScript = func() {
        return;
    }
    fmt.Println(fmt.Sprintf("%v", exitScript(&[_]string{ "1" })));
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 6
    st = 0;
    return;
}