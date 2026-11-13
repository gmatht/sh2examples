package main

import (
    "fmt"
    "os"
    "path/filepath"
    "strings"
    "time"
)

func main() {
    var current_date string
    var current_dir string
    var st int64

    fmt.Println("=== Basic Command Substitution ===");
    st = 0;
    fmt.Println(fmt.Sprintf("Current date: %v", time.Now().Format("2006"))); // line 9
    st = 0;
    fmt.Println(fmt.Sprintf("Current directory: %v", func() string { if strings.Join(strings.Fields(func() string { __pwdt1, __pwdet1 := os.Getwd(); if __pwdet1 != nil { return "" }; return __pwdt1 }()), " ") == "" { return "" }; return filepath.Base(strings.Join(strings.Fields(func() string { __pwdt1, __pwdet1 := os.Getwd(); if __pwdet1 != nil { return "" }; return __pwdt1 }()), " ")) }())); // line 11
    st = 0;
    current_date = time.Now().Format("200601"); // line 14
    st = 0;
    current_dir = func() string { if strings.Join(strings.Fields(func() string { __pwdt3, __pwdet3 := os.Getwd(); if __pwdet3 != nil { return "" }; return __pwdt3 }()), " ") == "" { return "" }; return filepath.Base(strings.Join(strings.Fields(func() string { __pwdt3, __pwdet3 := os.Getwd(); if __pwdet3 != nil { return "" }; return __pwdt3 }()), " ")) }(); // line 16
    st = 0;
    fmt.Println(fmt.Sprintf("Stored date: %v", current_date)); // line 18
    st = 0;
    fmt.Println(fmt.Sprintf("Stored directory: %v", current_dir)); // line 20
    st = 0;
    fmt.Println("=== Basic Command Substitution Complete ===");
    st = 0;
    _ = st
    os.Exit(int(st));
}