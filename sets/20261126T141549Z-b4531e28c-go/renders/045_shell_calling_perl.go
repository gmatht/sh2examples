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

func redirRun(script string, env []string) int64 {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    return runCmd(c)
}

func main() {
    var SHELL_VAR any
    var st int64

    fmt.Println("Warmup 1"); // line 3
    st = redirRun("'echo' 'apple' | 'perl' '-ne' 'print \"Fruit: $_\\n\"'", []string{"SHELL_VAR=" + s2s(SHELL_VAR)});
    fmt.Println("=== Example 1: Simple Perl one-liner ===");
    st = 0;
    st = runCmd(exec.Command("perl", "-e", "print \"Hello from Perl!\\n\"")); // line 12
    fmt.Println("\n=== Example 2: Perl with arguments ===");
    st = 0;
    st = runCmd(exec.Command("perl", "-e", "foreach $arg (@ARGV) { print \"Argument: $arg\\n\" }", "first", "second", "third"));
    fmt.Println("\n=== Example 3: Perl processing shell variables ===");
    st = 0;
    SHELL_VAR = "Hello World";
    st = 0;
    st = runCmd(exec.Command("perl", "-e", "print \"Shell variable: $ENV{SHELL_VAR}\\n\"")); // line 21
    os.Setenv("SHELL_VAR", s2s(SHELL_VAR)); // line 22
    st = runCmd(exec.Command("perl", "-e", "print \"Shell variable: $ENV{SHELL_VAR}\\n\"")); // line 23
    fmt.Println("\n=== Example 4: Perl reading from pipeline ===");
    st = 0;
    st = redirRun("'echo' '-e' 'apple\\nbanana\\ncherry' | 'perl' '-ne' 'chomp; print \"Fruit: $_\\n\"'", []string{"SHELL_VAR=" + s2s(SHELL_VAR)});
    fmt.Println("\n=== Example 5: Perl script with here document ===");
    st = 0;
    st = redirRun("( 'perl' ) 0<<'EOF'\nuse strict;\nuse warnings;\n\nmy @numbers = (1, 2, 3, 4, 5);\nmy $sum = 0;\n\nforeach my $num (@numbers) {\n    $sum += $num;\n    print \"Added $num, sum is now $sum\\n\";\n}\n\nprint \"Final sum: $sum\\n\";\n\nEOF", []string{"SHELL_VAR=" + s2s(SHELL_VAR)}); // line 31
    _ = st
    return;
}