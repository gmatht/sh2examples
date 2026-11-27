package main

import (
    "fmt"
)

func main() {
    fmt.Println("Testing ambiguous operators...");
    fmt.Println(fmt.Sprintf("2**3**2 = %v", 512));
    return;
}