package main

import (
    "fmt"
    "os"
)

func main() {
    switch { // line 4
        case os.Getenv("prev") == "$(echo \"foo|bar\")":
            fmt.Println("matched");
        default:
            _ = 0;
    }
    return;
}