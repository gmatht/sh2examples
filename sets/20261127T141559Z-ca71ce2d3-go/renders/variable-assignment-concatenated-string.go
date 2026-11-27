package main

import (
    "fmt"
    "os"
    "strings"
)

func main() {
    var dest_root string

    dest_root = fmt.Sprintf("%vp3", os.Getenv("emmccheck")); // line 3
    fmt.Println(fmt.Sprintf("%v", strings.Join(strings.Fields(dest_root), " "))); // line 4
    return;
}