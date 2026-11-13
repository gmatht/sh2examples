static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn __sh_fnmatch(pat: &str, s: &str) -> bool {
    // bash's pattern matcher treats the pattern as a C string — a NUL
    // truncates it (so `*$'\x00'*` is effectively just `*` and always
    // matches)
    let pat = pat.split('\0').next().unwrap_or(pat);
    let p: Vec<char> = pat.chars().collect();
    let t: Vec<char> = s.chars().collect();
    fn m(p: &[char], t: &[char]) -> bool {
        if p.is_empty() { return t.is_empty(); }
        match p[0] {
            '*' => {
                for i in 0..=t.len() { if m(&p[1..], &t[i..]) { return true; } }
                false
            }
            '?' => !t.is_empty() && m(&p[1..], &t[1..]),
            '[' => {
                if t.is_empty() { return false; }
                let mut j = 1;
                let neg = j < p.len() && (p[j] == '!' || p[j] == '^');
                if neg { j += 1; }
                let mut matched = false;
                let mut first = true;
                while j < p.len() && (p[j] != ']' || first) {
                    first = false;
                    if j + 2 < p.len() && p[j + 1] == '-' && p[j + 2] != ']' {
                        if t[0] >= p[j] && t[0] <= p[j + 2] { matched = true; }
                        j += 3;
                    } else {
                        if t[0] == p[j] { matched = true; }
                        j += 1;
                    }
                }
                if j < p.len() && p[j] == ']' { j += 1; }
                if matched != neg { m(&p[j..], &t[1..]) } else { false }
            }
            '\\' => p.len() > 1 && !t.is_empty() && t[0] == p[1] && m(&p[2..], &t[1..]),
            c if (c == '@' || c == '?' || c == '*' || c == '+' || c == '!')
                && p.len() > 1 && p[1] == '(' => {
                // extglob: parse the group's alternatives
                let mut depth = 1;
                let mut j = 2;
                let mut alts: Vec<Vec<char>> = Vec::new();
                let mut cur: Vec<char> = Vec::new();
                while j < p.len() && depth > 0 {
                    match p[j] {
                        '(' => { depth += 1; cur.push(p[j]); }
                        ')' => {
                            depth -= 1;
                            if depth == 0 {
                                alts.push(cur.clone());
                            } else { cur.push(p[j]); }
                        }
                        '|' if depth == 1 => { alts.push(std::mem::take(&mut cur)); }
                        ch => cur.push(ch),
                    }
                    j += 1;
                }
                if depth != 0 { return false; }
                let rest = &p[j..];
                match c {
                    '@' => {
                        for a in &alts {
                            let mut joined = a.clone();
                            joined.extend_from_slice(rest);
                            if m(&joined, t) { return true; }
                        }
                        false
                    }
                    '?' => {
                        if m(rest, t) { return true; }
                        for a in &alts {
                            let mut joined = a.clone();
                            joined.extend_from_slice(rest);
                            if m(&joined, t) { return true; }
                        }
                        false
                    }
                    '*' => {
                        // *(alts)rest  =  rest  |  a ++ *(alts)rest  (some alt a)
                        if m(rest, t) { return true; }
                        for a in &alts {
                            for i in 1..=t.len() {
                                if m(a, &t[..i]) && m(p, &t[i..]) { return true; }
                            }
                        }
                        false
                    }
                    '+' => {
                        // +(alts)rest  =  a ++ *(alts)rest  (at least one alt a)
                        for a in &alts {
                            for i in 1..=t.len() {
                                if m(a, &t[..i]) && m(p, &t[i..]) { return true; }
                            }
                        }
                        false
                    }
                    _ => {
                        // !(alts)rest: t = t1 ++ t2 with t2 matching rest
                        // and t1 matching NO alternative
                        for i in 0..=t.len() {
                            if m(rest, &t[i..]) {
                                let mut ok = true;
                                for a in &alts {
                                    if m(a, &t[..i]) { ok = false; break; }
                                }
                                if ok { return true; }
                            }
                        }
                        false
                    }
                }
            }
            c => !t.is_empty() && t[0] == c && m(&p[1..], &t[1..]),
        }
    }
    m(&p, &t)
}

fn __sh_replace(s: &str, pat0: &str, repl: &str, all: bool) -> String {
    let anchored_s = pat0.starts_with('#');
    let anchored_e = pat0.starts_with('%');
    let pat = pat0.trim_start_matches('#').trim_start_matches('%');
    let ch: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut done = false;
    while i <= ch.len() {
        if anchored_s && i > 0 { break; }
        let mut matched: Option<usize> = None;
        let mut end = i;
        while end <= ch.len() {
            let sub: String = ch[i..end].iter().collect();
            if __sh_fnmatch(pat, &sub) { matched = Some(end); break; }
            end += 1;
        }
        if let Some(e) = matched {
            if anchored_e && e != ch.len() { matched = None; }
        }
        if let Some(e) = matched {
            out.push_str(repl);
            i = e;
            done = true;
            if !all {
                out.extend(ch[i..].iter());
                break;
            }
            if i >= ch.len() { break; }
        } else {
            if i < ch.len() { out.push(ch[i]); }
            i += 1;
            if anchored_s { break; }
        }
    }
    if done { out } else { s.to_string() }
}

fn main() {
    let input = { let __v = { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() }; if __v.is_empty() { "default_value".to_string() } else { __v.to_string() } };
    let sanitized = __sh_replace(&input.clone(), "[^a-zA-Z0-9]", "_", true);
    let uppercase = sanitized.clone().to_uppercase();
    println!("{}", format!("Input: '{}' -> Sanitized: '{}' -> Uppercase: '{}'", input.clone(), sanitized.clone(), uppercase.clone()));
}