package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "path/filepath"
    "regexp"
    "sort"
    "strconv"
    "strings"
    "unicode/utf8"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func s2i(x any) int64 {
    s := strings.TrimSpace(fmt.Sprintf("%v", x))
    if n, err := strconv.ParseInt(s, 0, 64); err == nil { return n }
    return 0
}

func runeLen(s string) int64 {
    return int64(utf8.RuneCountInString(s))
}

func arrIdx(a []any, i int64) string {
    if i >= 0 && int(i) < len(a) { return s2s(a[i]) }
    return ""
}

func ifsJoin(out []string, isStar bool) string {
    if !isStar { return strings.Join(out, " ") }
    // STAR joins on the first char of IFS (empty IFS joins empty).
    if ifs, ok := vars["IFS"].(string); ok {
        if ifs == "" { return strings.Join(out, "") }
        return strings.Join(out, string([]rune(ifs)[0]))
    }
    return strings.Join(out, " ")
}

func arrSlice(a []any, start, end int64) []any {
    s, e := int(start), int(end)
    if s < 0 { s = len(a) + s }
    if e < 0 { e = len(a) + e }
    if s < 0 { s = 0 }
    if s > len(a) { s = len(a) }
    if e > len(a) { e = len(a) }
    if e < s { e = s }
    return a[s:e]
}

var fArgs []string

var shArgOff int

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1+shArgOff { return os.Args[1+shArgOff:] }
    return nil
}

func paramAt(i int) string {
    if i == 0 {
        if len(os.Args) > 0 { return os.Args[0] }
        return ""
    }
    a := argsList()
    if i >= 1 && i <= len(a) { return a[i-1] }
    return ""
}

func capRun(script string, env []string) string {
    c := exec.Command("bash", append([]string{"-c", script}, shArgs()...)...)
    shStdIn(c)
    shStdErr(c)
    c.Env = append(os.Environ(), env...)
    out, err := c.Output()
    // bash keeps what the command WROTE even when it exits nonzero
    // (`$(grep -c pat f1 f2)` prints the counts and takes status 1),
    // so a failed run is only empty when nothing was written.
    if err != nil && len(out) == 0 { return "" }
    return strings.TrimRight(string(out), "\n")
}

// shArgs: the parent positionals as a `bash -c` argv tail (the
// parent's $0, then argsList(): fArgs in a function, script args
// otherwise), so a reconstructed fragment naming `$@` observes the
// same params the parent does.
func shArgs() []string {
    if len(os.Args) > 0 {
        return append([]string{os.Args[0]}, argsList()...)
    }
    return append([]string{"sh2"}, argsList()...)
}

// pipeStatus is bash's PIPESTATUS: the exit code of EVERY stage of the
// last pipeline (not just the last one).
var pipeStatus []int64

func splitTop(s string) []string {
    var out []string
    depth := 0
    start := 0
    for i := 0; i < len(s); i++ {
        if s[i] == '(' { depth++ }
        if s[i] == ')' { depth-- }
        if s[i] == '|' && depth == 0 { out = append(out, s[start:i]); start = i+1 }
    }
    return append(out, s[start:])
}

func globRe(pat string, greedy bool) string {
    // An extglob head (`@(a|b)`, `*(x)`, `+(y)`, `?(z)`, `!(w)`) compiles
    // to its regex directly: `@` = one of, `*`/`+` = repeated, `?` =
    // optional, `!` = none-of (no lookahead — the caller splits the
    // `!(X)rest` shape out before calling). Alternation depth is
    // tracked so nested parens survive.
    if len(pat) >= 2 && pat[1] == '(' {
        switch pat[0] {
        case '@', '*', '+', '?', '!':
            depth := 0
            end := -1
            for i := 1; i < len(pat); i++ {
                if pat[i] == '(' { depth++ }
                if pat[i] == ')' { depth--; if depth == 0 { end = i; break } }
            }
            if end > 0 {
                inner := pat[2:end]
                rest := pat[end+1:]
                alts := splitTop(inner)
                var bx strings.Builder
                bx.WriteString("(?:")
                for ai, a := range alts {
                    if ai > 0 { bx.WriteString("|") }
                    bx.WriteString(globRe(a, true))
                }
                bx.WriteString(")")
                switch pat[0] {
                case '@': {}
                case '*': bx.WriteString("*")
                case '+': bx.WriteString("+")
                case '?': bx.WriteString("?")
                case '!': {
                    return "(?:(?!" + bx.String()[3:] + ")" + globRe(rest, greedy) + ")"
                }
                }
                bx.WriteString(globRe(rest, greedy))
                return bx.String()
            }
        default: {}
        }
    }
    var b strings.Builder
    for i := 0; i < len(pat); i++ {
        switch pat[i] {
        case '*':
            if greedy { b.WriteString(".*") } else { b.WriteString(".*?") }
        case '?': b.WriteString(".")
        case '[':
            // Bracket expression — `${p##*[/\\]}` needs the class, not an
            // escaped literal `[`. `[!…]` negates (bash), `[:class:]`
            // passes through (Go's regexp takes the same syntax). A
            // backslash is a MEMBER, not an escape: bash matches
            // `case \\ in [a\\]b) …` on both a, \\ and b, so the closing
            // `]` is the first one that is not the opening member.
            j := i + 1
            if j < len(pat) && (pat[j] == '!' || pat[j] == '^') { j++ }
            if j < len(pat) && pat[j] == ']' { j++ }
            for j < len(pat) && pat[j] != ']' { j++ }
            if j >= len(pat) { b.WriteString("\\["); break }
            body := pat[i+1 : j]
            b.WriteByte('[')
            // `[^…]` negates like `[!…]` (bash accepts both when the
            // character is FIRST): leaving `^` in the body made it a
            // MEMBER, so `${v//[^a-z0-9]/_}` replaced the alphanumerics
            // instead of the punctuation.
            if strings.HasPrefix(body, "!") || strings.HasPrefix(body, "^") { b.WriteString("^"); body = body[1:] }
            for k := 0; k < len(body); k++ {
                c := body[k]
                if c == '[' && k+1 < len(body) && body[k+1] == ':' {
                    if e := strings.Index(body[k:], ":]"); e >= 0 {
                        b.WriteString(body[k : k+e+2]); k += e + 1; continue
                    }
                }
                if c == ']' || c == '^' || c == '\\' { b.WriteByte('\\') }
                b.WriteByte(c)
            }
            b.WriteByte(']')
            i = j
        case '\\':
            // A backslash QUOTES the character after it: a backslash-paren
            // is a literal paren, a doubled backslash a literal backslash.
            // Emitting the backslash itself as a literal and then handling
            // the next char as a global instead turned the escaped-paren
            // pattern in `063_11` into "backslash, paren, ...", so every
            // such `case` pattern demanded real backslashes in the subject
            // and matched nothing.
            if i+1 < len(pat) {
                i++
                if pat[i] < 0x80 { b.WriteString(regexp.QuoteMeta(string(pat[i]))) } else { b.WriteByte(pat[i]) }
            } else {
                b.WriteString("\\\\")
            }
        case '.', '(', ')', '+', '|', '^', '$', ']', '{', '}':
            b.WriteByte('\\'); b.WriteByte(pat[i])
        default: b.WriteByte(pat[i])
        }
    }
    return b.String()
}

func pStrip(shortest bool, v, pat string) string {
    re := regexp.MustCompile("^(?:" + globRe(pat, !shortest) + ")")
    loc := re.FindStringIndex(v)
    if loc == nil { return v }
    return v[loc[1]:]
}

func pSuf(shortest bool, v, pat string) string {
    // `%`/`%%` remove a suffix, so the pattern is ANCHORED AT THE END:
    // `${HOME%/}` on /root keeps the whole value (there is no trailing
    // slash). An unanchored `^(.*)pat.*?$` matched the LEADING slash
    // and returned the empty string instead — the same shape broke every
    // `${v%…}`/`${v%%…}` whose pattern also occurs earlier in the value.
    pre := "(.*)"
    if !shortest { pre = "(.*?)" }
    re := regexp.MustCompile("(?s)^" + pre + globRe(pat, true) + "$")
    m := re.FindStringSubmatchIndex(v)
    if m == nil { return v }
    return v[:m[3]]
}

func pRep(v, pat, rep string, all bool) string {
    re := regexp.MustCompile(globRe(pat, true))
    if all { return re.ReplaceAllString(v, rep) }
    l := re.FindStringIndex(v)
    if l == nil { return v }
    return v[:l[0]] + rep + v[l[1]:]
}

var vars = map[string]any{}

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
func isDigits(s string) bool {
    if s == "" { return false }
    for i := 0; i < len(s); i++ {
        if s[i] < '0' || s[i] > '9' { return false }
    }
    return true
}

func atoi(s string) int {
    n, _ := strconv.Atoi(s)
    return n
}

// `bashVarNames` are SHELL variables: bash sets them but never exports
// them, so os.Getenv cannot see them and `${BASH_VERSION-}` would read as
// unset. They name the emulation itself, so their value is a constant of
// THIS translation rather than something to ask some other bash on the
// box for (a subprocess here would also make the render non-deterministic
// across hosts).
var bashVarNames = map[string]bool{
    "BASH_VERSION": true,
    "HOSTNAME": true,
}

func bashVar(name string) any {
    switch name {
    case "BASH_VERSION":
        return "5.2.21(1)-release"
    case "HOSTNAME":
        h, err := os.Hostname()
        if err != nil { return "" }
        return h
    }
    return ""
}

func pValRaw(name string) any {
    if v, ok := vars[name]; ok { return v }
    if bashVarNames[name] { return bashVar(name) }
    return os.Getenv(name)
}

func pVal(name string) string {
    // A bare positional (`${2:-d}` arrives as name "2"): shell vars
    // are never all-digits, so this is always a positional read — and
    // the value lives in fArgs/os.Args, never in the vars map.
    if isDigits(name) { return paramAt(atoi(name)) }
    if strings.HasPrefix(name, "#") {
        n := name[1:]
        if strings.HasSuffix(n, "[@]") {
            a, _ := pValRaw(n[:len(n)-3]).([]any)
            return strconv.FormatInt(int64(len(a)), 10)
        }
        if isDigits(n) { return strconv.FormatInt(runeLen(paramAt(atoi(n))), 10) }
        if a, ok := pValRaw(n).([]any); ok {
            return strconv.FormatInt(int64(len(a)), 10)
        }
        return strconv.FormatInt(runeLen(s2s(pValRaw(n))), 10)
    }
    if strings.HasSuffix(name, "[@]") || strings.HasSuffix(name, "[*]") {
        isStar := strings.HasSuffix(name, "[*]")
        base := name[:len(name)-3]
        if m, ok := pValRaw(base).(map[string]any); ok {
            keys := make([]string, 0, len(m))
            for k := range m { keys = append(keys, k) }
            sort.Strings(keys)
            out := make([]string, 0, len(m))
            for _, k := range keys { out = append(out, s2s(m[k])) }
            return ifsJoin(out, isStar)
        }
        a, _ := pValRaw(base).([]any)
        out := make([]string, 0, len(a))
        for _, v := range a { out = append(out, s2s(v)) }
        return ifsJoin(out, isStar)
    }
    // The vars map is keyed by FULL name (`vars["map[foo]"]` is what
    // the assoc write stores), so consult it before decomposing — the
    // decomposition below only knows whole-array lookups. A whole
    // array under its bare name reads as element zero (bash), not the
    // joined list — the [@]/[*] forms are the ones that join.
    if v, ok := vars[name]; ok {
        if a, ok := v.([]any); ok { return s2s(arrIdx(a, 0)) }
        return s2s(v)
    }
    if i := strings.Index(name, "["); i > 0 && strings.HasSuffix(name, "]") {
        // PIPESTATUS is not a shell ARRAY in the vars map — it is the
        // status slice the last pipeline left behind; reading it through
        // pValRaw made every `${PIPESTATUS[n]}` empty.
        if name[:i] == "PIPESTATUS" || name[:i] == "pipestatus" {
            k, err := strconv.ParseInt(name[i+1:len(name)-1], 0, 64)
            if err != nil { return "" }
            if k < 0 || k >= int64(len(pipeStatus)) { return "" }
            return strconv.FormatInt(pipeStatus[k], 10)
        }
        a, _ := pValRaw(name[:i]).([]any)
        k, err := strconv.ParseInt(name[i+1:len(name)-1], 0, 64)
        if err != nil {
            if e := expand(name[i+1 : len(name)-1]); e != name[i+1:len(name)-1] {
                if k2, err2 := strconv.ParseInt(strings.TrimSpace(e), 0, 64); err2 == nil {
                    return s2s(arrIdx(a, k2))
                }
                m, _ := pValRaw(name[:i]).(map[string]any)
                return s2s(m[strings.TrimSpace(e)])
            }
            m, _ := pValRaw(name[:i]).(map[string]any)
            return s2s(m[name[i+1:len(name)-1]])
        }
        return s2s(arrIdx(a, k))
    }
    return s2s(pValRaw(name))
}

func pSliceIdx(s string) (int64, bool) {
    if v, err := strconv.ParseInt(strings.TrimSpace(s), 0, 64); err == nil {
        return v, true
    }
    // A dynamic offset: expand it through the live store, then parse.
    if e := strings.TrimSpace(expand(s)); e != strings.TrimSpace(s) {
        if v, err := strconv.ParseInt(e, 0, 64); err == nil {
            return v, true
        }
    }
    return 0, false
}

// pIsSet is the UNSET test the `${x-d}` / `${x=d}` / `${x+d}` / `${x?d}`
// operator family turns on. `vars` is only this render's own store, and a
// name can be set without being in it: a bash SHELL variable
// (`bashVarNames`) is always set, and an exported one lives only in the
// environment. LookupEnv — not Getenv — is what separates set-but-empty
// from unset, which is the entire difference between these operators and
// their `:`-forms.
func pIsSet(name string) bool {
    if isDigits(name) { return int64(len(fArgs)) >= int64(atoi(name)) }
    if _, ok := vars[name]; ok { return true }
    if bashVarNames[name] { return true }
    _, ok := os.LookupEnv(name)
    return ok
}

func pExp(op, name string, args ...string) string {
    v := pVal(name)
    switch op {
    case "":
        return v
    case "^":
        if v == "" { return v }
        return strings.ToUpper(v[:1]) + v[1:]
    case "^^":
        return strings.ToUpper(v)
    case ",":
        if v == "" { return v }
        return strings.ToLower(v[:1]) + v[1:]
    case ",,":
        return strings.ToLower(v)
    case "#":
        if len(args) > 0 { return pStrip(true, v, args[0]) }
        return v
    case "##":
        if len(args) > 0 { return pStrip(false, v, args[0]) }
        return v
    case "%":
        if len(args) > 0 { return pSuf(true, v, args[0]) }
        return v
    case "%%":
        if len(args) > 0 { return pSuf(false, v, args[0]) }
        return v
    case "/":
        if len(args) >= 2 { return pRep(v, args[0], args[1], false) }
        if len(args) >= 1 { return pRep(v, args[0], "", false) }
        return v
    case "//":
        if len(args) >= 2 { return pRep(v, args[0], args[1], true) }
        if len(args) >= 1 { return pRep(v, args[0], "", true) }
        return v
    case "len":
        return strconv.FormatInt(runeLen(v), 10)
    case "basename":
        return filepath.Base(v)
    case "dirname":
        return filepath.Dir(v)
    case "slice":
        // `${@:n}`/`${*:n}` slice the POSITIONAL list, 1-based (the
        // same list `argsList` renders for `$@` in any other form).
        if name == "@" || name == "*" {
            a := argsList()
            st := int64(1)
            rest := args
            if len(rest) > 0 && (rest[0] == "@" || rest[0] == "*") { rest = rest[1:] }
            if len(rest) > 0 && strings.TrimSpace(rest[0]) != "" {
                if n, ok := pSliceIdx(rest[0]); ok { st = n }
            }
            if st < 1 { st = 1 }
            i := st - 1
            e := int64(len(a))
            if len(rest) > 1 && strings.TrimSpace(rest[1]) != "" {
                if ln, ok := pSliceIdx(rest[1]); ok { e = i + ln }
            }
            if i > int64(len(a)) { i = int64(len(a)) }
            if e > int64(len(a)) { e = int64(len(a)) }
            if e < i { e = i }
            out := make([]string, 0, e-i)
            for _, x := range a[i:e] { out = append(out, x) }
            return strings.Join(out, " ")
        }
        // An array value slices by ELEMENT: the bare name arrives for
        // both spellings, so branch on the runtime value.
        if a, ok := pValRaw(name).([]any); ok {
            rest := args
            if len(rest) > 0 && (rest[0] == "@" || rest[0] == "*") { rest = rest[1:] }
            st, _ := pSliceIdx("")
            if len(rest) > 0 { st, _ = pSliceIdx(rest[0]) }
            e := int64(len(a))
            if len(rest) > 1 && strings.TrimSpace(rest[1]) != "" {
                if ln, ok := pSliceIdx(rest[1]); ok { e = st+ln } else { e = st }
            }
            a = arrSlice(a, st, e)
            out := make([]string, 0, len(a))
            for _, v := range a { out = append(out, s2s(v)) }
            return strings.Join(out, " ")
        }
        st := int64(0)
        if len(args) > 0 { st, _ = pSliceIdx(args[0]) }
        if st < 0 { st = int64(runeLen(v)) + st }
        if st > int64(runeLen(v)) { st = int64(runeLen(v)) }
        if st < 0 { st = 0 }
        rs := []rune(v)
        if len(args) < 2 || strings.TrimSpace(args[1]) == "" { return string(rs[st:]) }
        en := st
        if ln, ok := pSliceIdx(args[1]); ok { en = st + ln }
        if en < 0 { en = int64(len(rs)) + en }
        if en > int64(len(rs)) { en = int64(len(rs)) }
        if en < st { en = st }
        return string(rs[st:en])
    case ":-":
        if v == "" && len(args) > 0 { return expand(args[0]) }
        return v
    case "-":
        // UNSET-only: `${x-d}` keeps a set-but-empty x (unlike `${x:-d}`).
        if !pIsSet(name) && len(args) > 0 { return expand(args[0]) }
        return v
    case "=":
        if !pIsSet(name) && len(args) > 0 {
            d := expand(args[0])
            vars[name] = d
            return d
        }
        return v
    case ":=":
        if v == "" && len(args) > 0 {
            d := expand(args[0])
            vars[name] = d
            return d
        }
        return v
    case ":?":
        if v == "" {
            if len(args) > 0 { _ = expand(args[0]) }
            os.Exit(1)
        }
        return v
    case "?":
        if !pIsSet(name) {
            if len(args) > 0 { _ = expand(args[0]) }
            os.Exit(1)
        }
        return v
    case "+":
        if !pIsSet(name) { return "" }
        if len(args) > 0 { return expand(args[0]) }
        return v
    }
    return v
}

func expand(s string) string {
    // A quoted default arrives with its quotes as data: strip one layer.
    if len(s) >= 2 {
        if (s[0] == '"' && s[len(s)-1] == '"') || (s[0] == '\'' && s[len(s)-1] == '\'') {
            s = s[1 : len(s)-1]
        }
    }
    var out strings.Builder
    for i := 0; i < len(s); {
        if s[i] == '$' && i+1 < len(s) {
            if s[i+1] == '(' {
                d := 1
                j := i + 2
                for j < len(s) && d > 0 {
                    if s[j] == '(' { d++ }
                    if s[j] == ')' { d-- }
                    j++
                }
                out.WriteString(capRun(s[i+2:j-1], nil))
                i = j
                continue
            }
            if s[i+1] == '{' {
                j := i + 2
                depth := 1
                for j < len(s) && depth > 0 {
                    if s[j] == '$' && j+1 < len(s) && s[j+1] == '{' { depth++; j += 2; continue }
                    if s[j] == '}' { depth-- };
                    if depth == 0 { break };
                    j++
                }
                if j >= len(s) { out.WriteString(s[i:]); break }
                out.WriteString(pExpStr(s[i+2 : j]))
                i = j + 1
                continue
            }
            j := i + 1
            for j < len(s) && (s[j] >= 'a' && s[j] <= 'z' || s[j] >= 'A' && s[j] <= 'Z' || s[j] >= '0' && s[j] <= '9' || s[j] == '_') { j++ }
            out.WriteString(pVal(s[i+1 : j]))
            i = j
            continue
        }
        out.WriteByte(s[i])
        i++
    }
    return out.String()
}

func pExpStr(inner string) string {
    best := -1
    depth := 0
    i := 0
    for i < len(inner) {
        if inner[i] == '$' && i+1 < len(inner) && inner[i+1] == '{' { depth++; i += 2; continue }
        if inner[i] == '}' {
            if depth > 0 { depth--; i++; continue }
            break
        }
        if depth == 0 && i > 0 {
            for _, op := range []string{":-", ":=", ":?", "##", "%%", "//", "^^", ",,", "#", "%", "/", "^", ",", "-", "+", "=", "?"} {
                if strings.HasPrefix(inner[i:], op) { best = i; break }
            }
            if best >= 0 { break }
        }
        i++
    }
    // The table is longest-first within each family, so the first
    // depth-zero match wins — except `#`/`%`/`/` also prefix their
    // doubles: prefer the longest op at the winning spot.
    if best >= 0 {
        bestOp := ""
        for _, op := range []string{"##", "%%", "//", "^^", ",,", ":-", ":=", ":?", "#", "%", "/", "^", ",", "-", "+", "=", "?"} {
            if strings.HasPrefix(inner[best:], op) { bestOp = op; break }
        }
        return pExp(bestOp, inner[:best], inner[best+len(bestOp):])
    }
    if strings.HasPrefix(inner, "#") { return pVal(inner) }
    return pVal(inner)
}

func bprintf(f string, a ...string) {
    fmt.Print(bprintfStr(f, a...))
}

func bprintfStr(f string, a ...string) string {
    f = bEsc(f)
    if len(a) == 0 { return f }
    var out strings.Builder
    for len(a) > 0 {
        var used int
        out.WriteString(bfmtOnce(f, a, &used))
        if used == 0 { break }
        a = a[used:]
    }
    return out.String()
}

func bfmtOnce(f string, a []string, used *int) string {
    var out strings.Builder
    for i := 0; i < len(f); i++ {
        c := f[i]
        if c != '%' || i+1 >= len(f) { out.WriteByte(c); continue }
        j := i + 1
        for j < len(f) && strings.ContainsRune("-+ #0", rune(f[j])) { j++ }
        for j < len(f) && f[j] >= '0' && f[j] <= '9' { j++ }
        if j < len(f) && f[j] == '.' {
            j++
            for j < len(f) && f[j] >= '0' && f[j] <= '9' { j++ }
        }
        for j < len(f) && strings.ContainsRune("lhLzjt", rune(f[j])) { j++ }
        if j >= len(f) { out.WriteByte('%'); break }
        v := f[j]
        spec := f[i : j+1]
        i = j
        switch v {
        case '%': out.WriteByte('%')
        case 's', 'q', 'd', 'i', 'u', 'o', 'x', 'X', 'e', 'E', 'f', 'F', 'g', 'G', 'c': {
            arg := ""
            if *used < len(a) { arg = a[*used] }
            *used++
            switch v {
            case 's': out.WriteString(bWidth(spec, arg))
            case 'q': out.WriteString(bShQuote(arg))
            case 'c': if len(arg) > 0 { out.WriteByte(arg[0]) }
            default: out.WriteString(bWidth(spec, bNum(v, arg)))
            }
        }
        case 'b': {
            arg := ""
            if *used < len(a) { arg = a[*used] }
            *used++
            out.WriteString(bEsc(arg))
        }
        default: out.WriteString(spec)
        }
    }
    return out.String()
}

func bWidth(spec, val string) string {
    d := spec[1:]
    minus := strings.HasPrefix(d, "-")
    if minus { d = d[1:] }
    w := 0
    for len(d) > 0 && d[0] >= '0' && d[0] <= '9' { w = w*10 + int(d[0]-'0'); d = d[1:] }
    if w <= len(val) { return val }
    pad := strings.Repeat(" ", w-len(val))
    if minus { return val + pad }
    return pad + val
}

// bShQuote is `printf %q`: the word rewritten so the shell reads it
// back unchanged. bash leaves printable safe characters bare, escapes
// every other printable with a backslash (`a b` -> `a\ b`), and uses
// the ANSI-C form when the text carries control characters
// (`$'\n\t'`) — `strconv.Quote` produces the last case's Go spelling,
// which is not a shell word.
func bShQuote(s string) string {
    if s == "" { return "''" }
    ansi := false
    for i := 0; i < len(s); i++ {
        if c := s[i]; c < 0x20 || c == 0x7f { ansi = true; break }
    }
    var b strings.Builder
    if ansi {
        b.WriteString("$'")
        for i := 0; i < len(s); i++ {
            c := s[i]
            switch c {
            case 7: b.WriteString("\\a")
            case 8: b.WriteString("\\b")
            case 12: b.WriteString("\\f")
            case '\n': b.WriteString("\\n")
            case '\r': b.WriteString("\\r")
            case '\t': b.WriteString("\\t")
            case 11: b.WriteString("\\v")
            case '\\': b.WriteString("\\\\")
            case '\'': b.WriteString("\\'")
            default:
                if c < 0x20 || c == 0x7f {
                    d := strconv.FormatInt(int64(c), 8)
                    b.WriteByte('\\')
                    for len(d) < 3 { d = "0" + d }
                    b.WriteString(d)
                } else {
                    b.WriteByte(c)
                }
            }
        }
        b.WriteString("'")
        return b.String()
    }
    const safe = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_@%+=:,./-"
    for i := 0; i < len(s); i++ {
        if strings.IndexByte(safe, s[i]) < 0 { b.WriteByte('\\') }
        b.WriteByte(s[i])
    }
    return b.String()
}

func bNum(v byte, arg string) string {
    n, err := strconv.ParseInt(strings.TrimSpace(arg), 0, 64)
    if err != nil { n = 0 }
    switch v {
    case 'd', 'i', 'u': return strconv.FormatInt(n, 10)
    case 'o': return strconv.FormatInt(n, 8)
    case 'x': return strconv.FormatInt(n, 16)
    case 'X': return strings.ToUpper(strconv.FormatInt(n, 16))
    case 'e', 'E', 'f', 'F', 'g', 'G': {
        fl, err := strconv.ParseFloat(strings.TrimSpace(arg), 64)
        if err != nil { fl = 0 }
        if v == 'e' || v == 'E' { return strconv.FormatFloat(fl, 'e', -1, 64) }
        if v == 'f' || v == 'F' { return strconv.FormatFloat(fl, 'f', -1, 64) }
        return strconv.FormatFloat(fl, 'g', -1, 64)
    }
    }
    return arg
}

func bEsc(f string) string {
    var out strings.Builder
    for i := 0; i < len(f); i++ {
        c := f[i]
        if c != '\\' || i+1 >= len(f) { out.WriteByte(c); continue }
        i++
        switch f[i] {
        case 'n': out.WriteByte('\n')
        case 't': out.WriteByte('\t')
        case 'r': out.WriteByte('\r')
        case 'a': out.WriteByte('\a')
        case 'b': out.WriteByte('\b')
        case 'f': out.WriteByte('\f')
        case 'v': out.WriteByte('\v')
        case '\\': out.WriteByte('\\')
        case '\'': out.WriteByte('\'')
        case '"': out.WriteByte('"')
        case 'e': out.WriteByte(0x1b)
        case 'c': return out.String()
        case '0', '1', '2', '3', '4', '5', '6', '7': {
            v := int(f[i] - '0')
            for k := 0; k < 2 && i+1 < len(f) && f[i+1] >= '0' && f[i+1] <= '7'; k++ {
                i++; v = v*8 + int(f[i]-'0')
            }
            out.WriteByte(byte(v))
        }
        default: out.WriteByte('\\'); out.WriteByte(f[i])
        }
    }
    return out.String()
}
func main() {
    var ch string
    var i string
    var len string
    var print_chars func() string
    var s string
    var st int64

    print_chars = func() string {
        s = paramAt(1); vars["s"] = s;
        st = 0;
        i = "0"; vars["i"] = i;
        st = 0;
        len = paramAt(2); vars["len"] = len;
        st = 0;
        __runt1 := false;
        for (s2i(i) < s2i(len)) {
            __runt1 = true;
            ch = pExp("slice", "s", "$i", "1"); vars["ch"] = ch;
            st = 0;
            bprintf("%s", ch);
            st = 0;
            i = strconv.FormatInt((s2i(i) + 1), 10); vars["i"] = i;
            st = 0;
        }
        if !__runt1 { st = 0; }
        fmt.Println("");
        st = 0;
        return "";
    }
    fArgs = []string{"HELLOWORLD", "10"}; print_chars(); fArgs = nil;
    _ = s
    _ = st
    os.Exit(int(st));
}