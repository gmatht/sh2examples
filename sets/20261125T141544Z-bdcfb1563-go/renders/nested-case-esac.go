package main

import (
    "fmt"
    "os"
)

var fArgs []string

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1 { return os.Args[1:] }
    return nil
}

func paramAt(i int) string {
    a := argsList()
    if i >= 1 && i <= len(a) { return a[i-1] }
    return ""
}

func main() {
    if (!(paramAt(1) != "start")) {
        fmt.Println("Starting...");
        if (!(paramAt(2) != "fast")) {
            fmt.Println("Fast start");
        } else if (!(paramAt(2) != "slow")) {
            fmt.Println("Slow start");
        }
    } else if (!(paramAt(1) != "stop")) {
        fmt.Println("Stopping...");
    } else if (!(paramAt(1) != "restart")) {
        fmt.Println("Restarting...");
        if (!(paramAt(2) != "soft")) {
            fmt.Println("Soft restart");
        } else if (!(paramAt(2) != "hard")) {
            fmt.Println("Hard restart");
        }
    }
    return;
}