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
    var divisor int64
    var factorize func()
    var factors any
    var n any

    fmt.Println("=== Number Factorization Examples ===");
    factorize = func() {
        n = paramAt(1);
        divisor = 2;
        factors = "";
        fmt.Print(fmt.Sprintf("Factors of %v: ", n));
        for (s2i(n) > 1) {
            for ((s2i(n) % divisor) == 0) {
                if (!(s2s(factors) != "")) {
                    factors = divisor;
                } else {
                    factors = fmt.Sprintf("%v * %v", factors, divisor);
                }
                n = (s2i(n) / divisor);
            }
            divisor = (divisor + 1);
            if ((divisor * divisor) > s2i(n)) {
                if (s2i(n) > 1) {
                    if (!(s2s(factors) != "")) {
                        factors = n;
                    } else {
                        factors = fmt.Sprintf("%v * %v", factors, n);
                    }
                }
                break;
            }
        }
        fmt.Println(fmt.Sprintf("%v", factors));
    }
    fArgs = []string{"12"}; factorize(); fArgs = nil; // line 44
    fArgs = []string{"28"}; factorize(); fArgs = nil; // line 45
    fArgs = []string{"100"}; factorize(); fArgs = nil; // line 46
    fArgs = []string{"12345"}; factorize(); fArgs = nil; // line 47
    fmt.Println("Factorization complete!");
    return;
}