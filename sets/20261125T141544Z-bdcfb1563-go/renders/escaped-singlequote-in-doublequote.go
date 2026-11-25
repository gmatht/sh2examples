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
    var conffile any
    var pkg any
    var x string

    pkg = "dummy"; // line 3
    conffile = "test"; // line 4
    x = s2s(capRun("'dpkg-query' '-W' '-f=${Conffiles}' \"$pkg\" | 'sed' '-n' '-e' \"\\\\' $conffile ' { s/ obsolete$//; s/.* //; p }\"", []string{"conffile=" + s2s(conffile), "pkg=" + s2s(pkg), "x=" + s2s(x)})); // line 5
    fmt.Print(fmt.Sprintf("result=[%v]\n", x)); // line 7
    fmt.Print("pkg=[dummy]\n");
    fmt.Print("conffile=[test]\n");
    _ = conffile
    _ = pkg
    return;
}