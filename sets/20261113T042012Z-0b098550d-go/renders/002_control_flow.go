package main

import (
    "fmt"
    "os"
    "strconv"
    "strings"
    "syscall"
)


func s2i(x any) int64 {
    s := strings.TrimSpace(fmt.Sprintf("%v", x))
    if n, err := strconv.ParseInt(s, 0, 64); err == nil { return n }
    return 0
}

var fArgs []string

var shArgOff int

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1+shArgOff { return os.Args[1+shArgOff:] }
    return nil
}

func paramAt(i int) string {
    if i == 0 {
        if len(os.Args) > 0 { return os.Args[0] }
        return ""
    }
    a := argsList()
    if i >= 1 && i <= len(a) { return a[i-1] }
    return ""
}

func fileTest(op string, p string) bool {
    st, err := os.Stat(p)
    switch op {
    case "e", "a": return err == nil
    case "f": return err == nil && st.Mode().IsRegular()
    case "d": return err == nil && st.IsDir()
    case "s": return err == nil && st.Size() > 0
    case "L", "h":
        ls, lerr := os.Lstat(p)
        return lerr == nil && ls.Mode()&os.ModeSymlink != 0
    case "p": return err == nil && st.Mode()&os.ModeNamedPipe != 0
    case "b": return err == nil && st.Mode()&os.ModeDevice != 0 && st.Mode()&os.ModeCharDevice == 0
    case "c": return err == nil && st.Mode()&os.ModeCharDevice != 0
    case "S": return err == nil && st.Mode()&os.ModeSocket != 0
    case "g": return err == nil && st.Mode()&os.ModeSetgid != 0
    case "k": return err == nil && st.Mode()&os.ModeSticky != 0
    case "u": return err == nil && st.Mode()&os.ModeSetuid != 0
    case "N": return false
    case "O":
        if err != nil { return false }
        if sys, ok := st.Sys().(*syscall.Stat_t); ok { return int(sys.Uid) == os.Getuid() }
        return false
    case "G":
        if err != nil { return false }
        if sys, ok := st.Sys().(*syscall.Stat_t); ok { return int(sys.Gid) == os.Getgid() }
        return false
    case "r": return syscall.Access(p, 4) == nil
    case "w": return syscall.Access(p, 2) == nil
    case "x": return syscall.Access(p, 1) == nil
    }
    return false
}
func main() {
    var greet func() string
    var i int64
    var st int64

    if fileTest("f", "file.txt") {
        fmt.Println("File exists");
        st = 0;
    } else {
        fmt.Println("File does not exist");
        st = 0;
    }
    __runt1 := false;
    for _, i = range []int64{s2i("1"), s2i("2"), s2i("3"), s2i("4"), s2i("5")} {
        __runt1 = true;
        fmt.Println(fmt.Sprintf("Number: %v", i));
        st = 0;
    }
    if !__runt1 { st = 0; }
    __runt2 := false;
    for (i < 10) {
        __runt2 = true;
        fmt.Println(fmt.Sprintf("Counter: %v", i));
        st = 0;
        i = (i + 1);
        st = 0;
    }
    if !__runt2 { st = 0; }
    greet = func() string {
        return fmt.Sprintf("Hello, %v!", paramAt(1));
    }
    fArgs = []string{"World"}; t3 := greet(); fArgs = nil;
    fmt.Println(fmt.Sprintf("%v", t3));
    st = 0;
    _ = st
    os.Exit(int(st));
}