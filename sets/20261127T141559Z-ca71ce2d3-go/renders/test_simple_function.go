package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
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

func capRun(script string, env []string) string {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func main() {
    var file any
    var get_file_size func()
    var size any

    get_file_size = func() { // line 3
        file = paramAt(1);
        size = s2s(strings.Join(strings.Fields(capRun("( 'wc' '-c' ) 0<${file}", []string{"file=" + s2s(file), "get_file_size=" + s2s(get_file_size)})), " "));
        fmt.Println(fmt.Sprintf("File %v has %v bytes", file, size));
    }
    fArgs = []string{"test_simple_function.sh"}; get_file_size(); fArgs = nil; // line 9
    return;
}