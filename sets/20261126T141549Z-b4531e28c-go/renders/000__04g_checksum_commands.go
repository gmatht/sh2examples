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

func runCmd(c *exec.Cmd) int64 {
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    if err := c.Run(); err != nil {
        if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
        return 1
    }
    return 0
}

func capCmd(c *exec.Cmd) string {
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
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
    var d string
    var sha256_result string
    var sha512_result string
    var strings_result string
    var st int64

    fmt.Println("=== Checksum Commands ===");
    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 10
    st = runCmd(exec.Command("cd", d)); // line 11
    if st != 0 {
        st = runCmd(exec.Command("exit", "1"));
    }
    os.WriteFile("test_checksum.txt", []byte("test content\n"), 0644);
    sha256_result = s2s(capCmd(exec.Command("sha256sum", "test_checksum.txt"))); // line 13
    st = 0;
    fmt.Println(fmt.Sprintf("SHA256 result: %v", sha256_result)); // line 14
    st = 0;
    sha512_result = s2s(capCmd(exec.Command("sha512sum", "test_checksum.txt"))); // line 18
    st = 0;
    fmt.Println(fmt.Sprintf("SHA512 result: %v", sha512_result)); // line 19
    st = 0;
    strings_result = s2s(capRun("'strings' 'target/debug/debashc.exe' | 'head' '-3'", []string{"d=" + s2s(d), "sha256_result=" + s2s(sha256_result), "sha512_result=" + s2s(sha512_result), "strings_result=" + s2s(strings_result)})); // line 23
    st = 0;
    fmt.Println("Strings result:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", strings_result)); // line 25
    st = 0;
    st = runCmd(exec.Command("rm", "-f", "test_checksum.txt")); // line 28
    fmt.Println("=== Checksum Commands Complete ===");
    st = 0;
    _ = os.Chdir("/"); // line 31
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 32
    _ = st
    return;
}