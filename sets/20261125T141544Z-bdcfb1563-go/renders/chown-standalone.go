package main

import (
    "fmt"
)

func main() {
    var st int64

    fmt.Println("chown root:root /tmp/testfile");
    st = 0;
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 5
    st = 0;
    return;
}