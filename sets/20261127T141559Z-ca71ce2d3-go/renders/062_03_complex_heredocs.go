package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Println("Testing complex here-documents...");
    st = 0;
    fmt.Print("This is a test line\nThis is not a test line\nThis is another test line\n");
    st = 0;
    fmt.Println(fmt.Sprintf("done: %v", st)); // line 11
    st = 0;
    return;
}