package main

import (
    "fmt"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func arrIdx(a []any, i int64) any {
    if i >= 0 && int(i) < len(a) { return a[i] }
    return nil
}

func main() {
    var FilesystemOptions []any

    FilesystemOptions = []any{"a", "b", "c"}; // line 2
    fmt.Println(fmt.Sprintf("%v", s2s(arrIdx(FilesystemOptions, ((2 * 1) - 1)))));
    return;
}