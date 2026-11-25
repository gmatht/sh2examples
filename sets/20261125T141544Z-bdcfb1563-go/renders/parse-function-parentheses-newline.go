package main

import (
    "fmt"
)

func main() {
    var myfunc func()
    var st int64

    myfunc = func() {
        return;
    }
    fmt.Println(fmt.Sprintf("%v", myfunc(&[_]string{  })));
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 9
    st = 0;
    return;
}