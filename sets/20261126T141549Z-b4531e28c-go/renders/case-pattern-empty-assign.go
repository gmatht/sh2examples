package main

import (
    "fmt"
    "os"
)

func main() {
    if (!(os.Getenv("x") != "a")) {
    }
    fmt.Print("parsed OK\n");
    return;
}