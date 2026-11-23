package main

import (
    "fmt"
    "os"
)

var fArgs []string

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1 { return os.Args[1:] }
    return nil
}

func paramAt(i int) string {
    a := argsList()
    if i >= 1 && i <= len(a) { return a[i-1] }
    return ""
}

func main() {
    var mul_xy func()
    var x string
    var y string

    mul_xy = func() {
        x = paramAt(1);
        y = paramAt(2);
        return;
    }
    fmt.Println(fmt.Sprintf("%v", mul_xy(&[_]string{ "3", "4" })));
    _ = x
    _ = y
    return;
}