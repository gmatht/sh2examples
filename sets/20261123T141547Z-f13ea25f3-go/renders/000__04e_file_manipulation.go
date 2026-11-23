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

func redirRun(script string, env []string) int64 {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    return runCmd(c)
}

func main() {
    var cp_result string
    var d string
    var mkdir_result string
    var mv_result string
    var rm_result string
    var touch_result string
    var st int64

    fmt.Println("=== File Manipulation Commands ===");
    d = s2s(capCmd(exec.Command("mktemp", "-d"))); // line 10
    st = runCmd(exec.Command("cd", d)); // line 11
    if st != 0 {
        st = runCmd(exec.Command("exit", "1"));
    }
    os.WriteFile("test_file.txt", []byte("test content\n"), 0644);
    cp_result = s2s(capRun("'cp' 'test_file.txt' 'test_file_copy.txt' && 'echo' 'Copy successful'", []string{"cp_result=" + s2s(cp_result), "d=" + s2s(d), "mkdir_result=" + s2s(mkdir_result), "mv_result=" + s2s(mv_result), "rm_result=" + s2s(rm_result), "touch_result=" + s2s(touch_result)}));
    st = 0;
    fmt.Println(fmt.Sprintf("Copy result: %v", cp_result)); // line 14
    st = 0;
    st = redirRun("( 'ls' 'test_file.txt' 'test_file_copy.txt' 'test_file_moved.txt' ) 2>/dev/null", []string{"cp_result=" + s2s(cp_result), "d=" + s2s(d), "mkdir_result=" + s2s(mkdir_result), "mv_result=" + s2s(mv_result), "rm_result=" + s2s(rm_result), "touch_result=" + s2s(touch_result)});
    if st != 0 {
        st = runCmd(exec.Command("echo", "No test files found"));
    }
    mv_result = s2s(capRun("'mv' 'test_file_copy.txt' 'test_file_moved.txt' && 'echo' 'Move successful'", []string{"cp_result=" + s2s(cp_result), "d=" + s2s(d), "mkdir_result=" + s2s(mkdir_result), "mv_result=" + s2s(mv_result), "rm_result=" + s2s(rm_result), "touch_result=" + s2s(touch_result)}));
    st = 0;
    fmt.Println(fmt.Sprintf("Move result: %v", mv_result)); // line 20
    st = 0;
    st = redirRun("( 'ls' 'test_file.txt' 'test_file_copy.txt' 'test_file_moved.txt' ) 2>/dev/null", []string{"cp_result=" + s2s(cp_result), "d=" + s2s(d), "mkdir_result=" + s2s(mkdir_result), "mv_result=" + s2s(mv_result), "rm_result=" + s2s(rm_result), "touch_result=" + s2s(touch_result)});
    if st != 0 {
        st = runCmd(exec.Command("echo", "No test files found"));
    }
    rm_result = s2s(capRun("'rm' 'test_file.txt' 'test_file_moved.txt' && 'echo' 'Remove successful'", []string{"cp_result=" + s2s(cp_result), "d=" + s2s(d), "mkdir_result=" + s2s(mkdir_result), "mv_result=" + s2s(mv_result), "rm_result=" + s2s(rm_result), "touch_result=" + s2s(touch_result)}));
    st = 0;
    fmt.Println(fmt.Sprintf("Remove result: %v", rm_result)); // line 26
    st = 0;
    st = redirRun("( 'ls' 'test_file.txt' 'test_file_copy.txt' 'test_file_moved.txt' ) 2>/dev/null", []string{"cp_result=" + s2s(cp_result), "d=" + s2s(d), "mkdir_result=" + s2s(mkdir_result), "mv_result=" + s2s(mv_result), "rm_result=" + s2s(rm_result), "touch_result=" + s2s(touch_result)});
    if st != 0 {
        st = runCmd(exec.Command("echo", "No test files found"));
    }
    mkdir_result = s2s(capRun("'mkdir' 'test_dir' && 'echo' 'Directory created'", []string{"cp_result=" + s2s(cp_result), "d=" + s2s(d), "mkdir_result=" + s2s(mkdir_result), "mv_result=" + s2s(mv_result), "rm_result=" + s2s(rm_result), "touch_result=" + s2s(touch_result)}));
    st = 0;
    fmt.Println(fmt.Sprintf("Mkdir result: %v", mkdir_result)); // line 32
    st = 0;
    st = runCmd(exec.Command("touch", "test_dir/file")); // line 33
    st = redirRun("( 'ls' 'test_dir' ) 2>/dev/null", []string{"cp_result=" + s2s(cp_result), "d=" + s2s(d), "mkdir_result=" + s2s(mkdir_result), "mv_result=" + s2s(mv_result), "rm_result=" + s2s(rm_result), "touch_result=" + s2s(touch_result)});
    if st != 0 {
        st = runCmd(exec.Command("echo", "Directory not found"));
    }
    touch_result = s2s(capRun("'touch' 'test_file.txt' && 'echo' 'File touched'", []string{"cp_result=" + s2s(cp_result), "d=" + s2s(d), "mkdir_result=" + s2s(mkdir_result), "mv_result=" + s2s(mv_result), "rm_result=" + s2s(rm_result), "touch_result=" + s2s(touch_result)}));
    st = 0;
    fmt.Println(fmt.Sprintf("Touch result: %v", touch_result)); // line 39
    st = 0;
    st = runCmd(exec.Command("rm", "-f", "test_file.txt", "test_file_copy.txt", "test_file_moved.txt")); // line 42
    st = redirRun("( 'rm' '-r' '-f' 'test_dir' ) 2>/dev/null", []string{"cp_result=" + s2s(cp_result), "d=" + s2s(d), "mkdir_result=" + s2s(mkdir_result), "mv_result=" + s2s(mv_result), "rm_result=" + s2s(rm_result), "touch_result=" + s2s(touch_result)}); // line 43
    if st != 0 {
        st = runCmd(exec.Command("true"));
    }
    fmt.Println("=== File Manipulation Commands Complete ===");
    st = 0;
    _ = os.Chdir("/"); // line 46
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", d)); // line 47
    _ = st
    return;
}