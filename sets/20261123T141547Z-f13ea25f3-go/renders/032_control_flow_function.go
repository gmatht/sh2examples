package main

import (
    "fmt"
)

func main() {
    var greet func()

    greet = func() {
        return;
    }
    fmt.Println(fmt.Sprintf("%v", greet(&[_]string{ "World" })));
    return;
}