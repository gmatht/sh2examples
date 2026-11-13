package main

import (
    "fmt"
    "os"
    "path"
    "path/filepath"
    "sort"
    "strings"
    "time"
)


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
func main() {
    var st int64

    fmt.Println("== Basic brace expansion ==");
    st = 0;
    fmt.Println("1 2 3 4 5");
    st = 0;
    fmt.Println("a b c");
    st = 0;
    fmt.Println("00 02 04");
    st = 0;
    fmt.Println("== Advanced brace expansion ==");
    st = 0;
    fmt.Println("a1 a2 a3 b1 b2 b3 c1 c2 c3");
    st = 0;
    fmt.Println("1 3 5 7 9");
    st = 0;
    fmt.Println("a d g j m p s v y");
    st = 0;
    fmt.Println("== Practical examples ==");
    st = 0;
    __ttt1 := time.Now();
    st = 0;
    if __e := os.Chtimes("file_001.txt", __ttt1, __ttt1); __e != nil { if __f, __fe := os.OpenFile("file_001.txt", os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes("file_001.txt", __ttt1, __ttt1); __e != nil { st = 1 } } }
    if __e := os.Chtimes("file_002.txt", __ttt1, __ttt1); __e != nil { if __f, __fe := os.OpenFile("file_002.txt", os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes("file_002.txt", __ttt1, __ttt1); __e != nil { st = 1 } } }
    if __e := os.Chtimes("file_003.txt", __ttt1, __ttt1); __e != nil { if __f, __fe := os.OpenFile("file_003.txt", os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes("file_003.txt", __ttt1, __ttt1); __e != nil { st = 1 } } }
    if __e := os.Chtimes("file_004.txt", __ttt1, __ttt1); __e != nil { if __f, __fe := os.OpenFile("file_004.txt", os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes("file_004.txt", __ttt1, __ttt1); __e != nil { st = 1 } } }
    if __e := os.Chtimes("file_005.txt", __ttt1, __ttt1); __e != nil { if __f, __fe := os.OpenFile("file_005.txt", os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes("file_005.txt", __ttt1, __ttt1); __e != nil { st = 1 } } }
    st = 0; // line 21
    __lsft2 := []string{}; __lsdt2 := []string{}
    for _, __lsxt2 := range append([]string{}, globArgs("file_*.txt")...) {
        __lsit2, __lset2 := os.Stat(__lsxt2)
        if __lset2 != nil { st = 2 } else if __lsit2.IsDir() && true { __lsdt2 = append(__lsdt2, __lsxt2) } else { __lsft2 = append(__lsft2, __lsxt2) }
    }
    sort.Strings(__lsft2); sort.Strings(__lsdt2)
    for _, __lsxt2 := range __lsft2 { fmt.Println(__lsxt2) }
    for __lskt2, __lsxt2 := range __lsdt2 {
        if len(__lsft2)+len(__lsdt2) > 1 { if __lskt2 > 0 || len(__lsft2) > 0 { fmt.Println("") }; fmt.Println(__lsxt2 + ":") }
        __lsrt2, __lsret2 := os.ReadDir(__lsxt2)
        if __lsret2 != nil { st = 2; continue }
        __lsnt2 := []string{}
        for _, __lset2 := range __lsrt2 {
            __lsnmt2 := __lset2.Name()
            if strings.HasPrefix(__lsnmt2, ".") { continue }
            __lsnt2 = append(__lsnt2, __lsnmt2)
        }
        sort.Strings(__lsnt2)
        for _, __lsnmt2 := range __lsnt2 { fmt.Println(__lsnmt2) }
    }
    st = 0; // line 22
    for _, __rpt3 := range globArgs("file_*.txt") {
        if err := os.Remove(__rpt3); err != nil { st = 1 }
    }
    fmt.Println(fmt.Sprintf("brace_expand: %v", "1 2 3 4 5"));
    st = 0;
    _ = st
    os.Exit(int(st));
}