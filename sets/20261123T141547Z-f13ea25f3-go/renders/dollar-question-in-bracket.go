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

func capRun(script string, env []string) string {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func main() {
    var LIBSSL string
    var st int64

    LIBSSL = s2s(capRun("( 'dpkg-query' '-f' '${db:Status-Abbrev}' '-W' 'libssl1.0.?' ) 2>&1", []string{"LIBSSL=" + s2s(LIBSSL)})); // line 4
    if (st == 2) {
        fmt.Print(fmt.Sprintf("LIBSSL=[%v]\n", LIBSSL));
        st = 0;
        fmt.Println("error");
        st = 0;
    }
    return;
}