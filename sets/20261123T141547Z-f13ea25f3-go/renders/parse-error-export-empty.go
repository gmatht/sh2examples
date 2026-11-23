package main

import (
    "fmt"
    "os"
)

func main() {
    os.Setenv("ENV", ""); // line 3
    fmt.Print(fmt.Sprintf("ENV=[%v]\n", os.Getenv("ENV"))); // line 4
    return;
}