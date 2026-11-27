package main

import (
    "fmt"
    "os"
    "strings"
)

var fArgs []string

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1 { return os.Args[1:] }
    return nil
}

func main() {
    var a any
    var show func()

    show = func() { // line 3
        fmt.Println(fmt.Sprintf("count=%v", int64(len(argsList()))));
        fmt.Println(fmt.Sprintf("all=%v", strings.Join(argsList(), " ")));
        t1 := argsList();
        for _, t2 := range t1 {
            a = t2;
            fmt.Println(fmt.Sprintf("arg=%v", a));
        }
    }
    fArgs = []string{"one", "two three", "four"}; show(); fArgs = nil;
    return;
}