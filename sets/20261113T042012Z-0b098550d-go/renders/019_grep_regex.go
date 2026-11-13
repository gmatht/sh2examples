package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
    "sync"
)


var psProcs = map[string]*exec.Cmd{}
var psMu sync.Mutex

func psClean(path string) {
    psMu.Lock()
    c := psProcs[path]
    delete(psProcs, path)
    psMu.Unlock()
    if c != nil && c.Process != nil { c.Process.Kill(); c.Wait() }
    os.Remove(path)
}

// pipeStatus is bash's PIPESTATUS: the exit code of EVERY stage of the
// last pipeline (not just the last one).
var pipeStatus []int64

func exitCode(err error) int64 {
    if err == nil { return 0 }
    if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
    return 1
}

func runPipe(stages [][]string) int64 {
    var cmds []*exec.Cmd
    for _, st := range stages {
        c := exec.Command(st[0], st[1:]...)
        cmds = append(cmds, c)
    }
    if len(cmds) == 0 { return 0 }
    var pipes []io.Closer
    for i := 0; i < len(cmds)-1; i++ {
        p, err := cmds[i].StdoutPipe()
        if err != nil { return 1 }
        cmds[i+1].Stdin = p
        pipes = append(pipes, p)
    }
    cmds[len(cmds)-1].Stdout = os.Stdout
    // After the pipe wiring (StdoutPipe refuses an already-set Stdout):
    // unwired Stdin ends follow the current process stdin.
    for _, c := range cmds { if c.Stdin == nil { c.Stdin = os.Stdin } }
    codes := make([]int64, len(cmds))
    last := int64(0)
    for i, c := range cmds {
        if i < len(cmds)-1 {
            if err := c.Start(); err != nil { codes[i] = 1 }
        } else {
            last = exitCode(c.Run())
            codes[i] = last
        }
    }
    // The parent holds a read end of every inter-stage pipe: close
    // the parent copies after Run so an early-exiting downstream
    // stage (head) retires the producers instead of deadlocking them.
    for _, p := range pipes { p.Close() }
    for i := 0; i < len(cmds)-1; i++ {
        codes[i] = exitCode(cmds[i].Wait())
    }
    pipeStatus = codes
    return last
}

func eEsc(s string) string {
    var out strings.Builder
    for i := 0; i < len(s); i++ {
        c := s[i]
        if c != '\\' || i+1 >= len(s) { out.WriteByte(c); continue }
        i++
        switch s[i] {
        case 'n': out.WriteByte('\n')
        case 't': out.WriteByte('\t')
        case 'r': out.WriteByte('\r')
        case 'a': out.WriteByte('\a')
        case 'b': out.WriteByte('\b')
        case 'f': out.WriteByte('\f')
        case 'v': out.WriteByte('\v')
        case '\\': out.WriteByte('\\')
        case '0', '1', '2', '3', '4', '5', '6', '7': {
            v := int(s[i] - '0')
            for k := 0; k < 2 && i+1 < len(s) && s[i+1] >= '0' && s[i+1] <= '7'; k++ {
                i++; v = v*8 + int(s[i]-'0')
            }
            out.WriteByte(byte(v))
        }
        default: out.WriteByte('\\'); out.WriteByte(s[i])
        }
    }
    return out.String()
}
func main() {
    var __ps_tmp0 string
    var matched string
    var st int64

    st = runPipe([][]string{[]string{"echo", "foo123 bar456"}, []string{"grep", "-E", "(foo|bar)[0-9]+"}});
    st = runPipe([][]string{[]string{"echo", "a+b*c?"}, []string{"grep", "-F", "a+b*c?"}});
    st = runPipe([][]string{[]string{"echo", "word wordly subword"}, []string{"grep", "-w", "word"}});
    st = runPipe([][]string{[]string{"echo", "-e", "exact whole line\\npartial line"}, []string{"grep", "-x", "exact whole line"}});
    st = runPipe([][]string{[]string{"echo", "-e", "error message\\nwarning message\\ninfo message"}, []string{"grep", "-E", "error|warning"}});
    __ps_tmp0 = func() string { __mkft1, __mket1 := os.CreateTemp("", "tmp*"); if __mket1 != nil { return "" }; __mknt1 := __mkft1.Name(); __mkft1.Close(); return __mknt1 }();
    st = 0;
    if __wet2 := os.WriteFile(__ps_tmp0, []byte(fmt.Sprintf("%v\n", eEsc("error\\nwarning"))), 0644); __wet2 != nil { st = 1 } else { st = 0 }
    st = runPipe([][]string{[]string{"echo", "-e", "error\\nwarning"}, []string{"grep", "-f", __ps_tmp0}});
    psClean(__ps_tmp0);
    st = runPipe([][]string{[]string{"echo", "file123.txt backup456.bak"}, []string{"grep", "-E", "([a-z]+)([0-9]+)\\.([a-z]+)"}});
    matched = func() string { __hst3_c := exec.Command("grep", "-c", ".*"); __hst3_c.Stdin = strings.NewReader(("test" + "\n")); __hst3_c.Stderr = io.Discard; __hst3_o, __hst3_e := __hst3_c.Output(); _ = __hst3_e; if len(__hst3_o) == 0 { return "" }; return strings.TrimRight(string(__hst3_o), "\n") }();
    st = 0;
    fmt.Println(fmt.Sprintf("  grep_exit: %v", st)); // line 28
    st = 0;
    fmt.Println(fmt.Sprintf("  match_count: %v", matched)); // line 29
    st = 0;
    os.Exit(int(st));
}