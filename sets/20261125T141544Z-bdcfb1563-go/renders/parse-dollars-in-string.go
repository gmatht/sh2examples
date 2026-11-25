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

func main() {
    var i any

    t1 := argsList(); // line 2
    for _, t2 := range t1 {
        i = t2;
        fmt.Println(fmt.Sprintf("%v", i));
    }
    t3 := argsList(); // line 3
    for _, t4 := range t3 {
        i = t4;
        fmt.Println(fmt.Sprintf("%v", i));
    }
    return;
}