package main

import (
    "fmt"
    "os"
    "regexp"
    "strings"
)


func b2i(b bool) int64 {
    if b { return 1 }
    return 0
}

func globMatch(pattern, s string) bool {
    // BASH pattern semantics, not path semantics: in `[[ x == pat ]]` and
    // `case`, `*` matches ANY character including `/` (`[[ /usr/bin == * ]]`
    // is true), while filepath.Match reserves `/` as a SEPARATOR and cannot
    // match across it — `*$'\x00'*` (`**`) therefore failed on every
    // absolute path (realpath-cmdsub). globRe is the pattern→regex the
    // `#`/`%`/`//` expansions already use, so one matcher serves them all.
    // `(?s)`: a shell value is not a LINE — `*` matches a newline
    // (`printf '%s' "a\nb"` is one word to the pattern), whereas Go's
    // `.` stops at it and turned `*` into a per-line wildcard.
    re, err := regexp.Compile("(?s)^(?:" + globRe(pattern, true) + ")$")
    if err != nil { return false }
    return re.MatchString(s)
}

func reMatch(pattern, s string) bool {
    ok, _ := regexp.MatchString(pattern, s)
    return ok
}

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
func main() {
    var f1 string
    var f2 string
    var s string
    var word string
    var st int64

    fmt.Println("== [[ pattern and regex ]]");
    st = 0;
    s = "file.txt";
    st = 0;
    st = b2i(!(globMatch("*.txt", s))); // line 10
    if st == 0 {
        fmt.Println("pattern-match");
        st = 0;
    }
    st = b2i(!(reMatch("^file\\.[a-z]+$", s))); // line 11
    if st == 0 {
        fmt.Println("regex-match");
        st = 0;
    }
    fmt.Println("== extglob ==");
    st = 0;
    f1 = "file.js";
    st = 0;
    f2 = "thing.min.js";
    st = 0;
    st = b2i(!((strings.HasSuffix(f1, ".js") && !globMatch("*.min", strings.TrimSuffix(f1, ".js"))))); // line 16
    if st == 0 {
        fmt.Println("f1-ok");
        st = 0;
    }
    st = b2i(!((strings.HasSuffix(f2, ".js") && !globMatch("*.min", strings.TrimSuffix(f2, ".js"))))); // line 17
    if st != 0 {
        fmt.Println("f2-filtered");
        st = 0;
    }
    fmt.Println("== nocasematch ==");
    st = 0;
    word = "Foo";
    st = 0;
    st = b2i(!(strings.EqualFold(word, "foo"))); // line 21
    if st == 0 {
        fmt.Println("ci-match");
        st = 0;
    }
    _ = st
    os.Exit(int(st));
}