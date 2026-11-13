package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "regexp"
    "strings"
)


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
func main() {
    var matched string
    var st int64

    st = runPipe([][]string{[]string{"echo", "-e", "match1\\nmatch2\\nmatch3\\nmatch4"}, []string{"grep", "-m", "2", "match"}});
    st = runPipe([][]string{[]string{"echo", "text with pattern in it"}, []string{"grep", "-b", "pattern"}});
    if __wet1 := os.WriteFile("temp_file.txt", []byte("content\n"), 0644); __wet1 != nil { st = 1 } else { st = 0 }
    st = func() int64 { __gre, __gce := regexp.Compile("content"); if __gce != nil { return 2 }; __gd, __gde := os.ReadFile("temp_file.txt"); if __gde != nil { return 2 }; __gls := strings.Split(string(__gd), "\n"); if len(__gls) > 0 && __gls[len(__gls)-1] == "" { __gls = __gls[:len(__gls)-1] }; __gc := 0; __go := ""; for _, __gl := range __gls { __gm := __gre.MatchString(__gl); if __gm { __gc++; __go += __gl + "\n" } }; fmt.Print(__go); if __gc > 0 { return 0 }; return 1 }();
    st = func() int64 { __gre, __gce := regexp.Compile("content"); if __gce != nil { return 2 }; __gd, __gde := os.ReadFile("temp_file.txt"); if __gde != nil { return 2 }; __gls := strings.Split(string(__gd), "\n"); if len(__gls) > 0 && __gls[len(__gls)-1] == "" { __gls = __gls[:len(__gls)-1] }; __gc := 0; __go := ""; for _, __gl := range __gls { __gm := __gre.MatchString(__gl); if __gm { __gc++; __go += "temp_file.txt" + ":" + __gl + "\n" } }; fmt.Print(__go); if __gc > 0 { return 0 }; return 1 }();
    st = runPipe([][]string{[]string{"grep", "-Z", "-l", "pattern", "temp_file.txt"}, []string{"tr", "\\0", "\\n"}});
    st = runPipe([][]string{[]string{"echo", "text with pattern in it"}, []string{"grep", "--color=always", "pattern"}});
    if st != 0 {
        fmt.Println("Color not supported");
        st = 0;
    }
    st = func() int64 { __gre, __gce := regexp.Compile("pattern"); if __gce != nil { return 2 }; __gd, __gde := os.ReadFile("temp_file.txt"); if __gde != nil { return 2 }; __gls := strings.Split(string(__gd), "\n"); if len(__gls) > 0 && __gls[len(__gls)-1] == "" { __gls = __gls[:len(__gls)-1] }; __gc := 0; __go := ""; for _, __gl := range __gls { __gm := __gre.MatchString(__gl); if __gm { __gc++ } }; fmt.Print(__go); if __gc > 0 { return 0 }; return 1 }();
    if st == 0 {
        fmt.Println("found");
        st = 0;
    }
    if st != 0 {
        fmt.Println("not found");
        st = 0;
    }
    st = 0; // line 29
    if err := os.Remove("temp_file.txt"); err != nil { st = 1 }
    matched = func() string { __hst3_c := exec.Command("grep", "-c", ".*"); __hst3_c.Stdin = strings.NewReader(("test" + "\n")); __hst3_c.Stderr = io.Discard; __hst3_o, __hst3_e := __hst3_c.Output(); _ = __hst3_e; if len(__hst3_o) == 0 { return "" }; return strings.TrimRight(string(__hst3_o), "\n") }();
    st = 0;
    fmt.Println(fmt.Sprintf("  grep_exit: %v", st)); // line 32
    st = 0;
    fmt.Println(fmt.Sprintf("  match_count: %v", matched)); // line 33
    st = 0;
    os.Exit(int(st));
}