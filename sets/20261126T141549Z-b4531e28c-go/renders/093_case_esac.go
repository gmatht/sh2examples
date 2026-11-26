package main

import (
    "fmt"
)

func main() {
    if true {
        fmt.Println("Hi!");
    } else if false {
        fmt.Println("Bye!");
    } else {
        fmt.Println("Other");
    }
    return;
}