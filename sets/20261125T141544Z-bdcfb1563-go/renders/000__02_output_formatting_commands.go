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
    var perl_result string
    var sha256_result string
    var sha512_result string
    var strings_result string
    var tee_result string
    var st int64

    fmt.Println("=== Output and Formatting Commands ===");
    fmt.Println(fmt.Sprintf("Echo result: %v", "Hello from backticks"));
    fmt.Println(fmt.Sprintf("Printf result: %v", "Number: 42, String: test"));
    fmt.Println("=== Compression Commands ===");
    fmt.Println("=== Network Commands ===");
    fmt.Println("=== Process Management Commands ===");
    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 56
    st = runCmd(exec.Command("cd", d)); // line 57
    if st != 0 {
        st = runCmd(exec.Command("exit", "1"));
    }
    fmt.Println("=== Checksum Commands ===");
    st = 0;
    os.WriteFile("test_checksum.txt", []byte("test content\n"), 0644);
    sha256_result = s2s(capCmd(exec.Command("sha256sum", "test_checksum.txt"))); // line 67
    st = 0;
    fmt.Println(fmt.Sprintf("SHA256 result: %v", sha256_result)); // line 68
    st = 0;
    sha512_result = s2s(capCmd(exec.Command("sha512sum", "test_checksum.txt"))); // line 72
    st = 0;
    fmt.Println(fmt.Sprintf("SHA512 result: %v", sha512_result)); // line 73
    st = 0;
    strings_result = s2s(capRun("'strings' 'test_binary.txt' | 'head' '-3'", []string{"d=" + s2s(d), "perl_result=" + s2s(perl_result), "sha256_result=" + s2s(sha256_result), "sha512_result=" + s2s(sha512_result), "strings_result=" + s2s(strings_result), "tee_result=" + s2s(tee_result)})); // line 77
    st = 0;
    fmt.Println("Strings result:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", strings_result)); // line 79
    st = 0;
    fmt.Println("=== I/O Redirection Commands ===");
    st = 0;
    tee_result = s2s(capRun("'echo' 'test output' | 'tee' 'test_tee.txt'", []string{"d=" + s2s(d), "perl_result=" + s2s(perl_result), "sha256_result=" + s2s(sha256_result), "sha512_result=" + s2s(sha512_result), "strings_result=" + s2s(strings_result), "tee_result=" + s2s(tee_result)}));
    st = 0;
    fmt.Println(fmt.Sprintf("Tee result: %v", tee_result)); // line 86
    st = 0;
    fmt.Println("=== Perl Command ===");
    st = 0;
    perl_result = s2s(capCmd(exec.Command("perl", "-e", "print \"Hello from Perl\\n\""))); // line 92
    st = 0;
    fmt.Println(fmt.Sprintf("Perl result: %v", perl_result)); // line 93
    st = 0;
    st = runCmd(exec.Command("rm", "-f", "test_checksum.txt", "test_tee.txt")); // line 96
    _ = os.Chdir("/"); // line 97
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 98
    _ = st
    return;
}