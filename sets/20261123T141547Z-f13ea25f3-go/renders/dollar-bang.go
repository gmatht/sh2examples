package main

import (
    "fmt"
    "os"
)

func main() {
    fmt.Println(fmt.Sprintf("background: %v", os.Getenv("!"))); // line 3
    return;
}