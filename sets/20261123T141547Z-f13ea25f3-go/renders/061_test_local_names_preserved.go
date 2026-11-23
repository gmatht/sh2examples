package main

import (
    "fmt"
    "os"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
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
    var array_name any
    var first_number any
    var index any
    var input_string any
    var new_value any
    var operation any
    var replacement any
    var search_pattern any
    var second_number any
    var test_arrays func()
    var test_math func()
    var test_strings func()

    test_math = func() {
        first_number = paramAt(1);
        second_number = paramAt(2);
        operation = paramAt(3);
        if (!(s2s(operation) != "add")) {
            return;
        } else if (!(s2s(operation) != "subtract")) {
            return;
        } else if (!(s2s(operation) != "multiply")) {
            return;
        } else {
            return;
        }
    }
    test_strings = func() {
        input_string = paramAt(1);
        search_pattern = paramAt(2);
        replacement = paramAt(3);
        if (!(s2s(search_pattern) != "start")) {
            return;
        } else if (!(s2s(search_pattern) != "end")) {
            return;
        } else if (!(s2s(search_pattern) != "middle")) {
            return;
        } else {
            return;
        }
    }
    test_arrays = func() {
        array_name = paramAt(1);
        index = paramAt(2);
        new_value = paramAt(3);
        if (!(s2s(index) != "first")) {
            return;
        } else if (!(s2s(index) != "last")) {
            return;
        } else {
            return;
        }
    }
    fmt.Println(fmt.Sprintf("%v", test_math(&[_]string{ "10", "5", "add" })));
    fmt.Println(fmt.Sprintf("%v", test_math(&[_]string{ "10", "5", "multiply" })));
    fmt.Println(fmt.Sprintf("%v", test_strings(&[_]string{ "hello world", "start", "hi" })));
    fmt.Println(fmt.Sprintf("%v", test_strings(&[_]string{ "hello world", "end", "bye" })));
    fmt.Println(fmt.Sprintf("%v", test_arrays(&[_]string{ "my_array", "first", "new_value" })));
    fmt.Println(fmt.Sprintf("%v", test_arrays(&[_]string{ "my_array", "last", "final_value" })));
    _ = array_name
    _ = first_number
    _ = input_string
    _ = new_value
    _ = replacement
    _ = second_number
    return;
}