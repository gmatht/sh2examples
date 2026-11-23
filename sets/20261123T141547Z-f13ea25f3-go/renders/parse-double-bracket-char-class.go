package main

import (
    "fmt"
    "os"
    "regexp"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func reMatch(pattern, s string) bool {
    ok, _ := regexp.MatchString(pattern, s)
    return ok
}

func main() {
    if reMatch(s2s("[abc]"), s2s(os.Getenv("x"))) { // line 4
        fmt.Println("match");
    }
    return;
}