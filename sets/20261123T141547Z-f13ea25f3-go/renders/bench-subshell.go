package main

import (
    "fmt"
)

func main() {
    fmt.Println(fmt.Sprintf("cmdsub=%v", "sub-result"));
    fmt.Println("group-run"); // line 5
    return;
}