package main

import (
    "fmt"
    "os"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func main() {
    var p any

    p = "/usr/local/bin/example"; // line 5
    if (s2s(os.Getenv("p##*/")) == s2s("example")) {
        fmt.Println("it is example");
    }
    _ = p
    return;
}