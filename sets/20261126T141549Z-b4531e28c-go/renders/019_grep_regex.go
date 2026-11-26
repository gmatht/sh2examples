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
    var __ps_tmp0 string
    var matched string
    var st int64

    st = redirRun("'echo' 'foo123 bar456' | 'grep' '-E' '(foo|bar)[0-9]+'", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "matched=" + s2s(matched)});
    st = redirRun("'echo' 'a+b*c?' | 'grep' '-F' 'a+b*c?'", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "matched=" + s2s(matched)});
    st = redirRun("'echo' 'word wordly subword' | 'grep' '-w' 'word'", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "matched=" + s2s(matched)});
    st = redirRun("'echo' '-e' 'exact whole line\\npartial line' | 'grep' '-x' 'exact whole line'", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "matched=" + s2s(matched)});
    st = redirRun("'echo' '-e' 'error message\\nwarning message\\ninfo message' | 'grep' '-E' 'error|warning'", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "matched=" + s2s(matched)});
    // TODO(unsupported): capture
    __ps_tmp0 = s2s(0);
    st = 0;
    os.WriteFile(__ps_tmp0, []byte("error\nwarning\n"), 0644);
    st = redirRun("'echo' '-e' 'error\\nwarning' | 'grep' '-f' \"$__ps_tmp0\"", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "matched=" + s2s(matched)});
    // TODO(unsupported): stmt Exec { cmd: Str("rm", DoubleQuoted), args: [Str("-f", DoubleQuoted), Var("__ps_tmp0", None)], capture: None, redirects: [], env: [] }
    st = redirRun("'echo' 'file123.txt backup456.bak' | 'grep' '-E' '([a-z]+)([0-9]+)\\.([a-z]+)'", []string{"__ps_tmp0=" + s2s(__ps_tmp0), "matched=" + s2s(matched)});
    // TODO(unsupported): capture
    matched = s2s(0);
    st = 0;
    fmt.Println(fmt.Sprintf("  grep_exit: %v", st)); // line 28
    st = 0;
    fmt.Println(fmt.Sprintf("  match_count: %v", matched)); // line 29
    st = 0;
    return;
}
// 3 construct(s) lowered to TODO markers