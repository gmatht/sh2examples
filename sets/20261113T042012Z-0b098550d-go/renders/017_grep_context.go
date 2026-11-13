package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "path"
    "path/filepath"
    "sort"
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

// fsGlob walks the filesystem like the shell's pathname expansion:
// every component is matched by filepath.Match, a leading dot in a
// name never matches a wildcard, and the result is sorted. Go's
// filepath.Glob cannot express `./*.sh` (its Match REFUSES the `.` and
// `/` components outright), so a recursive walk is the only faithful
// form.
func fsGlob(pat string) []string {
    var out []string
    if pat == "" { return out }
    var walk func(dir, rest string)
    walk = func(dir, rest string) {
        for {
            i := strings.IndexByte(rest, '/')
            if i < 0 { break }
            comp := rest[:i]
            rest = rest[i+1:]
            if rest == "" { break }
            if !strings.ContainsAny(comp, "*?[\\") {
                if comp == "." { continue }
                if comp == ".." {
                    if dir == "." { dir = ".." } else { dir = filepath.Dir(dir) }
                    continue
                }
                dir = filepath.Join(dir, comp)
                continue
            }
            ents, _ := os.ReadDir(dir)
            for _, e := range ents {
                name := e.Name()
                if strings.HasPrefix(name, ".") && !strings.HasPrefix(comp, ".") { continue }
                if ok, _ := path.Match(comp, name); ok {
                    walk(filepath.Join(dir, name), rest)
                }
            }
            return
        }
        if !strings.ContainsAny(rest, "*?[\\") {
            p := rest
            if dir != "." || strings.HasPrefix(pat, "./") { p = filepath.Join(dir, rest) }
            if _, err := os.Lstat(p); err == nil { out = append(out, p) }
            return
        }
        entries, err := os.ReadDir(dir)
        if err != nil { return }
        for _, e := range entries {
            name := e.Name()
            if strings.HasPrefix(name, ".") && !strings.HasPrefix(rest, ".") { continue }
            if ok, _ := path.Match(rest, name); ok {
                p := filepath.Join(dir, name)
                if dir == "." && !strings.HasPrefix(pat, "./") { p = name }
                out = append(out, p)
            }
        }
    }
    start := "."
    if strings.HasPrefix(pat, "/") { start = "/"; pat = strings.TrimPrefix(pat, "/") }
    walk(start, pat)
    sort.Strings(out)
    return out
}

// globArgs expands a glob word into argv entries. filepath.Glob is tried
// FIRST: it is Go's own expansion and handles every pattern shape,
// including the dotfile rule (`.*` matches dotfiles, `*` does not).
// fsGlob is the fallback for the shapes filepath refuses outright
// (`.//x`, `dir/../x`).
func globArgs(pat string) []string {
    if m, err := filepath.Glob(pat); err == nil {
        // bash's dotfile rule: a wildcard never matches a LEADING dot
        // unless the pattern's own component starts with one (`.*`,
        // `.sh`). Go's filepath.Match has no such rule, so `*` returns
        // `.sh` where bash's `*` does not -- drop those.
        out := m[:0:0]
        for _, x := range m {
            if globDotOk(pat, x) { out = append(out, x) }
        }
        if len(out) > 0 { return out }
        // a no-match pattern is a LITERAL argument in bash (nullglob off)
        return []string{pat}
    }
    // filepath.Glob refused the pattern outright (a dot path component
    // or a wildcard in a directory position): walk it ourselves.
    // A pattern whose FIRST component begins with `.` (`.*`, `.??*`)
    // matches dotfiles: bash's rule is only that a wildcard never
    // matches a LEADING dot, and here the pattern asked for one. Without
    // this the dotfile filter below would UNDO filepath.Glob's correct
    // answer (measured: `grep -c ".*"` came back with every file).
    if p := pat; strings.HasPrefix(p, ".") && strings.ContainsAny(p, "*?[") {
        if m := fsGlob(pat); len(m) > 0 { return m }
    }
    // Absolute and `./`-rooted patterns walk the filesystem (bash can
    // expand them; filepath.Glob refuses them outright).
    if strings.HasPrefix(pat, "./") || strings.HasPrefix(pat, "../") {
        m := fsGlob(pat)
        if len(m) == 0 { return []string{pat} }
        return m
    }
    m, err := filepath.Glob(pat)
    if err != nil || m == nil { return []string{pat} }
    // bash does not let a wildcard match a leading dot: `*` yields no
    // dotfile unless the pattern's own component starts with `.`. Go's
    // filepath.Match HAS no such rule, so `ls * .sh` listed `.sh`
    // twice (once from `*`, once from the literal). Filter it back out;
    // an all-dotfile match leaves the pattern unexpanded, exactly as
    // bash does with nullglob off.
    // A pattern whose FIRST component begins with `.` (`.*`, `.??*`)
    // matches dotfiles: bash's rule is only that a wildcard never
    // matches a LEADING dot, and here the pattern asked for one. Without
    // this the dotfile filter below would UNDO filepath.Glob's correct
    // answer (measured: grep -c on a dotted pattern returned files).
    if p := pat; strings.HasPrefix(p, ".") && strings.ContainsAny(p, "*?[") {
        if m := fsGlob(pat); len(m) > 0 { return m }
    }

    // A pattern whose LAST component begins with `.` (`.sh`, `.*`)
    // matches dotfiles: bash's dot rule is about a LEADING dot in the
    // name, and the pattern asked for one explicitly. filepath.Glob
    // already returned them, so the dotfile filter must leave them.
    last := pat
    if i := strings.LastIndexByte(pat, '/'); i >= 0 { last = pat[i+1:] }
    // A pattern component that BEGINS with `.` asked for dotfiles (bash
    // only forbids a wildcard matching a leading dot), so filepath.Glob
    // already answered correctly -- the filter below must not undo it.
    if strings.HasPrefix(last, ".") { return m }
    for _, comp := range strings.Split(pat, "/") {
        if strings.HasPrefix(comp, ".") { return m }
    }
    if !strings.HasPrefix(last, ".") {
        out := m[:0:0]
        for _, s := range m {
            base := s
            if i := strings.LastIndexByte(s, '/'); i >= 0 { base = s[i+1:] }
            if !strings.HasPrefix(base, ".") { out = append(out, s) }
        }
        if len(out) == 0 { return []string{pat} }
        return out
    }
    return m
}

// globDotOk: does `pat` legitimately match the leading-dot path `x`?
// bash expands a wildcard over a name's leading dot only when the
// pattern's corresponding component itself starts with a dot.
func globDotOk(pat, x string) bool {
    pc := strings.Split(pat, "/")
    xc := strings.Split(x, "/")
    if len(pc) != len(xc) { return true }
    for i := range pc {
        if strings.HasPrefix(xc[i], ".") && !strings.HasPrefix(pc[i], ".") { return false }
    }
    return true
}

var origStderr = os.Stderr

// shStd wires a child to the CURRENT process stdio: a nil Stdin
    // follows the (possibly swapped) os.Stdin var, and a nil Stderr
    // follows a swapped os.Stderr but stays discarded otherwise
    // (native helpers never write stderr by default). Stdout is left
    // alone: capturers attach their own pipe (pre-setting it breaks
    // Output), and runners default it explicitly. Function bodies run
    // with these vars temporarily swapped for welded redirects, so
    // every spawn site funnels through here instead of inheriting the
    // raw fds (which the swap does not change).
func shStdIn(c *exec.Cmd) {
    if c.Stdin == nil { c.Stdin = os.Stdin }
}

func shStdErr(c *exec.Cmd) {
    if c.Stderr == nil {
        if os.Stderr != origStderr { c.Stderr = os.Stderr } else { c.Stderr = io.Discard }
    }
}
func main() {
    var d string
    var matched string
    var st int64

    st = runPipe([][]string{[]string{"echo", "-e", "line1\\nline2\\nTARGET\\nline4\\nline5"}, []string{"grep", "-A", "2", "TARGET"}});
    st = runPipe([][]string{[]string{"echo", "-e", "line1\\nline2\\nTARGET\\nline4\\nline5"}, []string{"grep", "-B", "2", "TARGET"}});
    st = runPipe([][]string{[]string{"echo", "-e", "line1\\nline2\\nTARGET\\nline4\\nline5"}, []string{"grep", "-C", "1", "TARGET"}});
    d = func() string { __mknt1, __mket1 := os.MkdirTemp("", "tmp*"); if __mket1 != nil { return "" }; return __mknt1 }(); // line 12
    st = 0;
    if err := os.Chdir(d); err != nil { st = 1 } else { st = 0 }; // line 13
    if st != 0 {
        os.Exit(int(1));
    }
    if __wet2 := os.WriteFile("temp_file1.txt", []byte("pattern in file1\n"), 0644); __wet2 != nil { st = 1 } else { st = 0 }
    if __wet3 := os.WriteFile("temp_file2.txt", []byte("no pattern in file2\n"), 0644); __wet3 != nil { st = 1 } else { st = 0 }
    if __wet4 := os.WriteFile("temp_file3.txt", []byte("pattern in file3\n"), 0644); __wet4 != nil { st = 1 } else { st = 0 }
    fmt.Println("Recursive search results:");
    st = 0;
    __ec5 := exec.Command("grep", append(append(append([]string{"-r"}, "pattern"), "."), globArgs("--include=*.txt")...)...)
    shStdIn(__ec5); shStdErr(__ec5); __ec5.Stdout = os.Stdout;
    st = 0
    if __ee5 := __ec5.Run(); __ee5 != nil { st = 127; if __ec5.ProcessState != nil { st = int64(__ec5.ProcessState.ExitCode()) } }
    fmt.Println("Result 2..."); // line 21
    st = 0;
    st = runPipe([][]string{append(append(append([]string{"grep"}, "-l"), "pattern"), globArgs("*.txt")...), []string{"sort"}});
    fmt.Println("Result 3..."); // line 25
    st = 0;
    __ec6 := exec.Command("grep", append(append([]string{"-L"}, "pattern"), globArgs("*.txt")...)...)
    shStdIn(__ec6); shStdErr(__ec6); __ec6.Stdout = os.Stdout;
    st = 0
    if __ee6 := __ec6.Run(); __ee6 != nil { st = 127; if __ec6.ProcessState != nil { st = int64(__ec6.ProcessState.ExitCode()) } }
    if err := os.Chdir("/"); err != nil { st = 1 } else { st = 0 }; // line 29
    st = 0; // line 30
    if err := os.RemoveAll(d); err != nil { st = 1 }
    matched = func() string { __hst8_c := exec.Command("grep", "-c", ".*"); __hst8_c.Stdin = strings.NewReader(("test" + "\n")); __hst8_c.Stderr = io.Discard; __hst8_o, __hst8_e := __hst8_c.Output(); _ = __hst8_e; if len(__hst8_o) == 0 { return "" }; return strings.TrimRight(string(__hst8_o), "\n") }();
    st = 0;
    fmt.Println(fmt.Sprintf("  grep_exit: %v", st)); // line 33
    st = 0;
    fmt.Println(fmt.Sprintf("  match_count: %v", matched)); // line 34
    st = 0;
    os.Exit(int(st));
}