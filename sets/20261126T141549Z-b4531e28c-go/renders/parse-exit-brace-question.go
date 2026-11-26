package main

import (
    "fmt"
    "os"
    "strconv"
    "strings"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func s2i(x any) int64 {
    s := strings.TrimSpace(fmt.Sprintf("%v", x))
    if n, err := strconv.ParseInt(s, 0, 64); err == nil { return n }
    return 0
}

func main() {
    var st int64

    os.Exit(int(s2i(strings.Join(strings.Fields(s2s(st)), " ")))); // line 3
    fmt.Print("parsed OK\n");
    st = 0;
    return;
}