package main

import (
    "fmt"
    "os"
)

func main() {
    fmt.Println(fmt.Sprintf("options: %v", os.Getenv("-"))); // line 3
    return;
}