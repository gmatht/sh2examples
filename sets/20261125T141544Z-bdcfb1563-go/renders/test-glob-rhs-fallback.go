package main

import (
    "fmt"
    "path/filepath"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func globMatch(pattern, s string) bool {
    ok, _ := filepath.Match(pattern, s)
    return ok
}

func main() {
    if globMatch(s2s("a*"), s2s("abc")) { // line 6
        fmt.Println("bad");
    } else {
        fmt.Println("literal-star-nomatch");
    }
    if globMatch(s2s("a*"), s2s("xyz")) { // line 7
        fmt.Println("bad");
    } else {
        fmt.Println("const-star-nomatch");
    }
    if globMatch(s2s("a*"), s2s("abc")) {
        fmt.Println("bad");
    } else {
        fmt.Println("var-star-nomatch");
    }
    if globMatch(s2s("a*"), s2s("abc")) { // line 11
        fmt.Println("bad");
    } else {
        fmt.Println("db-bracket-nomatch");
    }
    if globMatch(s2s("[ab]*"), s2s("abc")) { // line 13
        fmt.Println("bad");
    } else {
        fmt.Println("bracket-literal-nomatch");
    }
    return;
}