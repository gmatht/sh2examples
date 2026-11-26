package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
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

func main() {
    var __ps_tmp0 string
    var __ps_tmp1 string
    var __ps_tmp2 string
    var __ps_tmp3 string
    var st int64

    os.WriteFile("/tmp/cmp_a.txt", []byte("abcdefghij\n"), 0644);
    os.WriteFile("/tmp/cmp_same.txt", []byte("abcdefghij\n"), 0644);
    os.WriteFile("/tmp/cmp_diff.txt", []byte("abcdeZghij\n"), 0644);
    os.WriteFile("/tmp/cmp_diff2.txt", []byte("xyzdefghij\n"), 0644);
    os.WriteFile("/tmp/cmp_short.txt", []byte("abc\n"), 0644);
    st = runCmd(exec.Command("touch", "/tmp/cmp_empty.txt")); // line 11
    st = runCmd(exec.Command("cmp", "/tmp/cmp_a.txt", "/tmp/cmp_same.txt")); // line 16
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 17
    st = 0;
    st = runCmd(exec.Command("cmp", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt")); // line 20
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 21
    st = 0;
    st = runCmd(exec.Command("cmp", "/tmp/cmp_a.txt", "/tmp/cmp_empty.txt")); // line 24
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 25
    st = 0;
    st = runCmd(exec.Command("cmp", "/tmp/cmp_empty.txt", "/tmp/cmp_empty.txt")); // line 28
    fmt.Println(fmt.Sprintf("exit: %v", st)); // line 29
    st = 0;
    st = runCmd(exec.Command("cmp", "-s", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt")); // line 32
    fmt.Println(fmt.Sprintf("-s exit: %v", st)); // line 33
    st = 0;
    st = runCmd(exec.Command("cmp", "-s", "/tmp/cmp_a.txt", "/tmp/cmp_same.txt")); // line 34
    fmt.Println(fmt.Sprintf("-s same exit: %v", st)); // line 35
    st = 0;
    st = runCmd(exec.Command("cmp", "-l", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt")); // line 38
    fmt.Println(fmt.Sprintf("-l exit: %v", st)); // line 39
    st = 0;
    st = runCmd(exec.Command("cmp", "-b", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt")); // line 42
    fmt.Println(fmt.Sprintf("-b exit: %v", st)); // line 43
    st = 0;
    st = runCmd(exec.Command("cmp", "-n", "5", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt")); // line 47
    fmt.Println(fmt.Sprintf("-n 5 exit: %v", st)); // line 48
    st = 0;
    st = runCmd(exec.Command("cmp", "-n", "10", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt")); // line 50
    fmt.Println(fmt.Sprintf("-n 10 exit: %v", st)); // line 51
    st = 0;
    st = runCmd(exec.Command("cmp", "-n", "10", "/tmp/cmp_a.txt", "/tmp/cmp_short.txt")); // line 53
    fmt.Println(fmt.Sprintf("-n 10 short exit: %v", st)); // line 54
    st = 0;
    st = runCmd(exec.Command("cmp", "-i", "6", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt")); // line 58
    fmt.Println(fmt.Sprintf("-i 6 exit: %v", st)); // line 59
    st = 0;
    st = runCmd(exec.Command("cmp", "-i", "3", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt")); // line 61
    fmt.Println(fmt.Sprintf("-i 3 exit: %v", st)); // line 62
    st = 0;
    st = runCmd(exec.Command("cmp", "-i", "0:6", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt")); // line 66
    fmt.Println(fmt.Sprintf("-i 0:6 exit: %v", st)); // line 67
    st = 0;
    st = runCmd(exec.Command("cmp", "-i", "5:0", "/tmp/cmp_a.txt", "/tmp/cmp_diff2.txt")); // line 69
    fmt.Println(fmt.Sprintf("-i 5:0 exit: %v", st)); // line 70
    st = 0;
    // TODO(unsupported): capture
    __ps_tmp0 = s2s(0);
    st = 0;
    os.WriteFile(__ps_tmp0, []byte("a\n"), 0644);
    // TODO(unsupported): capture
    __ps_tmp1 = s2s(0);
    st = 0;
    os.WriteFile(__ps_tmp1, []byte("a\n"), 0644);
    st = runCmd(exec.Command("cmp", "-s", __ps_tmp0, __ps_tmp1));
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp0", None), Var("__ps_tmp1", None)], capture: None, redirects: [], env: [] }
    fmt.Println(fmt.Sprintf("aa -s exit: %v", st)); // line 74
    st = 0;
    // TODO(unsupported): capture
    __ps_tmp2 = s2s(0);
    st = 0;
    os.WriteFile(__ps_tmp2, []byte("b\n"), 0644);
    // TODO(unsupported): capture
    __ps_tmp3 = s2s(0);
    st = 0;
    os.WriteFile(__ps_tmp3, []byte("c\n"), 0644);
    st = runCmd(exec.Command("cmp", "-s", __ps_tmp2, __ps_tmp3));
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp2", None), Var("__ps_tmp3", None)], capture: None, redirects: [], env: [] }
    fmt.Println(fmt.Sprintf("bc -s exit: %v", st)); // line 76
    st = 0;
    st = runCmd(exec.Command("rm", "-f", "/tmp/cmp_a.txt", "/tmp/cmp_same.txt", "/tmp/cmp_diff.txt", "/tmp/cmp_diff2.txt", "/tmp/cmp_short.txt", "/tmp/cmp_empty.txt")); // line 79
    return;
}
// 6 construct(s) lowered to TODO markers