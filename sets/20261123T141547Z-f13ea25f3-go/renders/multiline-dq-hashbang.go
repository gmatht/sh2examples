package main

import (
    "fmt"
)

func main() {
    fmt.Println("#!/bin/sh\nexec 'command' --option \"$@\"");
    return;
}