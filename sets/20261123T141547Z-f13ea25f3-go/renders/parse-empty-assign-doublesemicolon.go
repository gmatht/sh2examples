package main

import (
    "fmt"
    "os"
)

func main() {
    if (!(os.Getenv("needop") != "")) {
        fmt.Println("empty");
    }
    return;
}