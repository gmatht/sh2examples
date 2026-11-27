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
    var a any

    fmt.Println("== Argument count ==");
    fmt.Println(fmt.Sprintf("%v", int64(len(argsList())))); // line 7
    fmt.Println("== Arguments ==");
    t1 := argsList(); // line 10
    for _, t2 := range t1 {
        a = t2;
        fmt.Println(fmt.Sprintf("Arg: %v", a));
    }
    return;
}