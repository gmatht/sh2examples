package main

import (
    "fmt"
)

func main() {
    switch { // line 3
        case "word" == "$(echo \"pattern\")":
            fmt.Println("matched");
        default:
            _ = 0;
    }
    return;
}