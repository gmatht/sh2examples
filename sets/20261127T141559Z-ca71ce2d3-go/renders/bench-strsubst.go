package main

import (
    "fmt"
)

func main() {
    fmt.Println(fmt.Sprintf("all=%v", "abc,def,ghi"));
    fmt.Println(fmt.Sprintf("front=%v", "XYZ:def:ghi"));
    fmt.Println(fmt.Sprintf("back=%v", "abc:def:XYZ"));
    return;
}