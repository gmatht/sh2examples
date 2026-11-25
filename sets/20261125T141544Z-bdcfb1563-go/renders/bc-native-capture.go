package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
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

func runCmd(c *exec.Cmd) int64 {
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    if err := c.Run(); err != nil {
        if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
        return 1
    }
    return 0
}

func capRun(script string, env []string) string {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func redirRun(script string, env []string) int64 {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    return runCmd(c)
}

func main() {
    var i int64
    var n int64
    var sum string
    var st int64

    st = redirRun("'echo' '2+3' | 'bc'", []string{"i=" + s2s(i), "n=" + s2s(n), "sum=" + s2s(sum)});
    st = redirRun("'echo' 'scale=2; 5/2' | 'bc'", []string{"i=" + s2s(i), "n=" + s2s(n), "sum=" + s2s(sum)});
    st = redirRun("'echo' '2^10' | 'bc'", []string{"i=" + s2s(i), "n=" + s2s(n), "sum=" + s2s(sum)});
    st = redirRun("'echo' '7 % 3' | 'bc'", []string{"i=" + s2s(i), "n=" + s2s(n), "sum=" + s2s(sum)});
    st = redirRun("'echo' '-2^2' | 'bc'", []string{"i=" + s2s(i), "n=" + s2s(n), "sum=" + s2s(sum)});
    st = redirRun("'echo' '0.5+0.25' | 'bc'", []string{"i=" + s2s(i), "n=" + s2s(n), "sum=" + s2s(sum)});
    st = redirRun("'echo' 'sqrt(25)' | 'bc'", []string{"i=" + s2s(i), "n=" + s2s(n), "sum=" + s2s(sum)});
    for _, n = range []int64{s2i("4"), s2i("9"), s2i("16"), s2i("25"), s2i("100")} { // line 17
        fmt.Println(fmt.Sprintf("sqrt(%v) = %v", n, capRun("'echo' \"sqrt($n)\" | 'bc'", []string{"i=" + s2s(i), "n=" + s2s(n), "sum=" + s2s(sum)})));
        st = 0;
    }
    sum = "0"; // line 21
    st = 0;
    for _, i = range []int64{s2i("1"), s2i("2"), s2i("3"), s2i("4"), s2i("5")} { // line 22
        sum = s2s(capRun("'echo' \"$sum + $i\" | 'bc'", []string{"i=" + s2s(i), "n=" + s2s(n), "sum=" + s2s(sum)}));
        st = 0;
    }
    fmt.Println(fmt.Sprintf("sum=%v", sum)); // line 25
    st = 0;
    _ = i
    _ = st
    return;
}