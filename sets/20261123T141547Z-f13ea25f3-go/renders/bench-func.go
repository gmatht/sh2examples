package main

import (
    "fmt"
)

func main() {
    var f func()

    f = func() {
        return;
    }
    fmt.Println(fmt.Sprintf("%v", f(&[_]string{  })));
    fmt.Println(fmt.Sprintf("%v", f(&[_]string{  })));
    return;
}