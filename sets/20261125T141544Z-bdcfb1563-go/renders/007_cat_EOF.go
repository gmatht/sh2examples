package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Print("alpha\nbeta\ngamma ...\n");
    st = 0;
    fmt.Print("oyster\nsnapper\nsalmon\n");
    st = 0;
    fmt.Println("Fin. That is all folks.");
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 15
    st = 0;
    return;
}