package main

import (
    "crypto/sha256"
    "crypto/sha512"
    "encoding/hex"
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
)


func capPipe(stages [][]string) string {
    return strings.TrimRight(capPipeRaw(stages), "\n")
}

// The capture form of runPipe: same native stage chain, but the LAST
// stage's stdout is read instead of inherited, so `$(cmd | cmd ...)` needs
// no `bash -c` child. Both the read and the Waits must happen before the
// last stage is reaped - a StdoutPipe is not valid after Wait.
func capPipeRaw(stages [][]string) string {
    var cmds []*exec.Cmd
    for _, st := range stages {
        if len(st) == 0 { return "" }
        c := exec.Command(st[0], st[1:]...)
        cmds = append(cmds, c)
    }
    if len(cmds) == 0 { return "" }
    var pipes []io.Closer
    for i := 0; i < len(cmds)-1; i++ {
        p, err := cmds[i].StdoutPipe()
        if err != nil { return "" }
        cmds[i+1].Stdin = p
        pipes = append(pipes, p)
    }
    last, err := cmds[len(cmds)-1].StdoutPipe()
    if err != nil { return "" }
    // After the pipe wiring: unwired Stdin ends follow the process stdin.
    for _, c := range cmds { if c.Stdin == nil { c.Stdin = os.Stdin } }
    for _, c := range cmds {
        if err := c.Start(); err != nil {
            for _, d := range cmds { if d.Process != nil { d.Process.Kill() } }
            for _, d := range cmds { d.Wait() }
            return ""
        }
    }
    data, err := io.ReadAll(last)
    if err != nil { data = nil }
    // Same parent-holds-the-pipe deadlock as runPipe (see above).
    for _, p := range pipes { p.Close() }
    for _, c := range cmds { c.Wait() }
    return string(data)
}
func main() {
    var d string
    var sha256_result string
    var sha512_result string
    var strings_result string
    var st int64

    fmt.Println("=== Checksum Commands ===");
    st = 0;
    d = func() string { __mknt1, __mket1 := os.MkdirTemp("", "tmp*"); if __mket1 != nil { return "" }; return __mknt1 }(); // line 10
    st = 0;
    if err := os.Chdir(d); err != nil { st = 1 } else { st = 0 }; // line 11
    if st != 0 {
        os.Exit(int(1));
    }
    if __wet2 := os.WriteFile("test_checksum.txt", []byte("test content\n"), 0644); __wet2 != nil { st = 1 } else { st = 0 }
    sha256_result = func() string { __cdt3, __cdet3 := os.ReadFile("test_checksum.txt"); if __cdet3 != nil { return "" }; __cht3 := sha256.Sum256(__cdt3); return hex.EncodeToString(__cht3[:]) + "  " + "test_checksum.txt" }(); // line 13
    st = 0;
    fmt.Println(fmt.Sprintf("SHA256 result: %v", sha256_result)); // line 14
    st = 0;
    sha512_result = func() string { __cdt4, __cdet4 := os.ReadFile("test_checksum.txt"); if __cdet4 != nil { return "" }; __cht4 := sha512.Sum512(__cdt4); return hex.EncodeToString(__cht4[:]) + "  " + "test_checksum.txt" }(); // line 18
    st = 0;
    fmt.Println(fmt.Sprintf("SHA512 result: %v", sha512_result)); // line 19
    st = 0;
    strings_result = capPipe([][]string{[]string{"strings", "target/debug/debashc.exe"}, []string{"head", "-3"}}); // line 23
    st = 0;
    fmt.Println("Strings result:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", strings_result)); // line 25
    st = 0;
    st = 0; // line 28
    if err := os.Remove("test_checksum.txt"); err != nil && !os.IsNotExist(err) { st = 1 }
    fmt.Println("=== Checksum Commands Complete ===");
    st = 0;
    if err := os.Chdir("/"); err != nil { st = 1 } else { st = 0 }; // line 31
    st = 0; // line 32
    if err := os.RemoveAll(d); err != nil { st = 1 }
    _ = st
    os.Exit(int(st));
}