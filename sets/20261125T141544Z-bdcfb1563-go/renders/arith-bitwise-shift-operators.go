package main

import (
    "fmt"
)

func main() {
    fmt.Println(fmt.Sprintf("%v", ((5 + 3) & 12))); // line 3
    fmt.Println(fmt.Sprintf("%v", (1 << 4))); // line 4
    fmt.Println(fmt.Sprintf("%v", (32 >> 2))); // line 5
    fmt.Println(fmt.Sprintf("%v", (7 | 8))); // line 6
    fmt.Println(fmt.Sprintf("%v", (12 ^ 5))); // line 7
    fmt.Println(fmt.Sprintf("%v", (^0))); // line 8
    return;
}