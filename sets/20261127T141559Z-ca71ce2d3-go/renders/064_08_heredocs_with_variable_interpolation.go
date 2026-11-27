package main

import (
    "fmt"
)

func main() {
    fmt.Print("User: $USER\nHost: ${HOSTNAME:-localhost}\nPath: $PWD\n"); // line 4
    fmt.Println("Config file created");
    return;
}