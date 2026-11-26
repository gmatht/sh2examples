package main

import (
    "fmt"
)

func powN(a, b int64) int64 {
    r := int64(1)
    for i := int64(0); i < b; i++ { r *= a }
    return r
}

func main() {
    var a int64
    var b int64
    var c int64
    var d int64
    var e int64
    var f int64
    var g int64
    var h int64
    var i int64
    var j int64
    var k int64
    var l int64
    var m int64
    var n int64
    var result any

    a = 10; // line 4
    b = 3;
    c = 8;
    d = 2;
    e = 7;
    f = 2;
    g = 2;
    h = 3;
    i = 4;
    j = 1;
    k = 6;
    l = 3;
    m = 5;
    n = 2;
    result = ((((((a + b) * (c - d)) / (e % f)) + powN(g,h)) - (i << j)) | ((k & l) ^ (m | n))); // line 5
    fmt.Println(fmt.Sprintf("Deeply nested arithmetic result: %v", result)); // line 6
    return;
}