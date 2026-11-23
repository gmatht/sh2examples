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
    var count string
    var found string
    var lineno string
    var longline string
    var matched string
    var st int64

    fmt.Println("== Basic grep parameters ==");
    st = redirRun("'echo' 'text with pattern' | 'grep' '-i' 'PATTERN'", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)});
    if st == 0 {
        st = runCmd(exec.Command("echo", "  -i match: OK"));
    }
    if st != 0 {
        st = runCmd(exec.Command("echo", "  -i match: FAIL"));
    }
    count = s2s(capRun("'echo' '-e' 'line1\\nline2\\nline3' | 'grep' '-v' 'line2' | 'wc' '-l'", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)}));
    st = 0;
    fmt.Println(fmt.Sprintf("  -v count: %v (expected 2)", count)); // line 10
    st = 0;
    matched = s2s(capRun("'echo' '-e' 'match\\nno match\\nmatch again' | 'grep' '-c' 'match'", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)}));
    st = 0;
    fmt.Println(fmt.Sprintf("  -c count: %v (expected 2)", matched)); // line 13
    st = 0;
    fmt.Println("== Context parameters ==");
    st = 0;
    st = redirRun("'echo' '-e' 'line1\\nline2\\nTARGET\\nline4\\nline5' | ( 'grep' '-A' '2' 'TARGET' ) >/tmp/grep_out.txt", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)});
    fmt.Println(fmt.Sprintf("  -A 2 lines: %v (expected 3)", capRun("( 'wc' '-l' ) 0</tmp/grep_out.txt", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)}))); // line 17
    st = 0;
    st = redirRun("'echo' '-e' 'line1\\nline2\\nTARGET\\nline4\\nline5' | ( 'grep' '-B' '2' 'TARGET' ) >/tmp/grep_out.txt", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)});
    fmt.Println(fmt.Sprintf("  -B 2 lines: %v (expected 3)", capRun("( 'wc' '-l' ) 0</tmp/grep_out.txt", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)}))); // line 20
    st = 0;
    st = redirRun("'echo' '-e' 'line1\\nline2\\nTARGET\\nline4\\nline5' | ( 'grep' '-C' '1' 'TARGET' ) >/tmp/grep_out.txt", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)});
    fmt.Println(fmt.Sprintf("  -C 1 lines: %v (expected 3)", capRun("( 'wc' '-l' ) 0</tmp/grep_out.txt", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)}))); // line 23
    st = 0;
    fmt.Println("== File handling parameters ==");
    st = 0;
    os.WriteFile("/tmp/grep_file.txt", []byte("content\n"), 0644);
    st = runCmd(exec.Command("grep", "-c", "content", "/tmp/grep_file.txt"));
    if st == 0 {
        st = runCmd(exec.Command("echo", "  -c file: OK"));
    }
    if st != 0 {
        st = runCmd(exec.Command("echo", "  -c file: FAIL"));
    }
    st = runCmd(exec.Command("grep", "-l", "content", "/tmp/grep_file.txt"));
    if st == 0 {
        st = runCmd(exec.Command("echo", "  -l: found"));
    }
    if st != 0 {
        st = runCmd(exec.Command("echo", "  -l: not found"));
    }
    st = runCmd(exec.Command("grep", "-L", "nonexistent", "/tmp/grep_file.txt"));
    if st == 0 {
        st = runCmd(exec.Command("echo", "  -L: not found (correct)"));
    }
    if st != 0 {
        st = runCmd(exec.Command("echo", "  -L: found (wrong)"));
    }
    fmt.Println("== Output formatting parameters ==");
    st = 0;
    // TODO(unsupported): capture
    matched = s2s(0);
    st = 0;
    fmt.Println(fmt.Sprintf("  -o match: '%v' (expected 'pattern')", matched)); // line 33
    st = 0;
    lineno = s2s(capRun("'echo' 'text with pattern in it' | 'grep' '-n' 'pattern' | 'cut' '-d' ':' '-f' '1'", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)}));
    st = 0;
    fmt.Println(fmt.Sprintf("  -n line: %v (expected 1)", lineno)); // line 36
    st = 0;
    fmt.Println("== Recursive parameters ==");
    st = 0;
    st = runCmd(exec.Command("mkdir", "-p", "/tmp/grep_sub"));
    if st == 0 {
        st = redirRun("( 'echo' 'subfile content' ) >/tmp/grep_sub/file.txt", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)});
    }
    found = s2s(capRun("( 'grep' '-r' 'subfile' '/tmp/grep_sub' ) 2>/dev/null | 'wc' '-l'", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)}));
    st = 0;
    fmt.Println(fmt.Sprintf("  -r recursive: %v files matched (expected 1)", found)); // line 41
    st = 0;
    st = runCmd(exec.Command("rm", "-r", "-f", "/tmp/grep_sub")); // line 42
    fmt.Println("== Line length parameters ==");
    st = 0;
    longline = s2s(capRun("echo -n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)}));
    st = 0;
    st = redirRun("'echo' \"$longline\" | ( 'grep' '-m' '1' 'a' ) >/dev/null", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)});
    if st == 0 {
        st = runCmd(exec.Command("echo", "  -m 1 (long line): OK"));
    }
    if st != 0 {
        st = runCmd(exec.Command("echo", "  -m 1 (long line): FAIL"));
    }
    fmt.Println("== Word-regexp and line-regexp parameters ==");
    st = 0;
    st = redirRun("'echo' '-e' 'foo\\nfoobar\\nbar' | ( 'grep' '-w' 'foo' ) >/tmp/grep_out.txt", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)});
    fmt.Println(fmt.Sprintf("  -w word match lines: %v (expected 1)", capRun("( 'wc' '-l' ) 0</tmp/grep_out.txt", []string{"count=" + s2s(count), "found=" + s2s(found), "lineno=" + s2s(lineno), "longline=" + s2s(longline), "matched=" + s2s(matched)}))); // line 50
    st = 0;
    st = runCmd(exec.Command("rm", "-f", "/tmp/grep_file.txt", "/tmp/grep_out.txt")); // line 52
    _ = longline
    _ = st
    return;
}
// 1 construct(s) lowered to TODO markers