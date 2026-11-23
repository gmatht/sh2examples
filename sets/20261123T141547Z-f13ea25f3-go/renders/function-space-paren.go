package main

import (
    "fmt"
)

func main() {
    var my_func func()
    var st int64

    my_func = func() {
        return;
    }
    fmt.Println(fmt.Sprintf("%v", my_func(&[_]string{  })));
    st = 0;
    fmt.Println(fmt.Sprintf("done: %v", st)); // line 9
    st = 0;
    return;
}