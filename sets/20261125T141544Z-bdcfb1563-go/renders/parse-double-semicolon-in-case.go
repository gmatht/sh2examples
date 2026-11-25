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
    var dryrun any

    if (!(paramAt(1) != "--dryrun")) {
        dryrun = "t";
    } else if (!(paramAt(1) != "a")) {
    } else if (!(paramAt(1) != "b")) {
        dryrun = "t";
    }
    fmt.Print("parsed OK\n");
    _ = dryrun
    return;
}