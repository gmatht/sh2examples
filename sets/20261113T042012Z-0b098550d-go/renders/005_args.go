package main

import (
    "fmt"
    "os"
)


var fArgs []string

var shArgOff int

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1+shArgOff { return os.Args[1+shArgOff:] }
    return nil
}
func main() {
    var a string
    var st int64

    fmt.Println("== Argument count ==");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", int64(len(argsList())))); // line 7
    st = 0;
    fmt.Println("== Arguments ==");
    st = 0;
    __runt1 := false; // line 10
    t2 := argsList();
    for _, t3 := range t2 {
        __runt1 = true;
        a = t3;
        fmt.Println(fmt.Sprintf("Arg: %v", a));
        st = 0;
    }
    if !__runt1 { st = 0; }
    _ = st
    os.Exit(int(st));
}