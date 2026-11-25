thread_local! { #[allow(non_upper_case_globals)] static __SHV___sh2_casesubj_0_: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___sh2_casesubj_1_: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_fnmatch(pat: &str, s: &str) -> bool {
    let pat = pat.split('\0').next().unwrap_or(pat);
    let p: Vec<char> = pat.chars().collect();
    let t: Vec<char> = s.chars().collect();
    fn class_pred(n: &str) -> Option<fn(char) -> bool> {
        Some(match n {
            "alnum" => |c: char| c.is_alphanumeric(),
            "alpha" => |c: char| c.is_alphabetic(),
            "blank" => |c: char| c == ' ' || c == '\t',
            "cntrl" => |c: char| c.is_control(),
            "digit" => |c: char| c.is_ascii_digit(),
            "graph" => |c: char| !c.is_whitespace() && !c.is_control(),
            "lower" => |c: char| c.is_lowercase(),
            "print" => |c: char| !c.is_control(),
            "punct" => |c: char| c.is_ascii_punctuation(),
            "space" => |c: char| c.is_whitespace(),
            "upper" => |c: char| c.is_uppercase(),
            "xdigit" => |c: char| c.is_ascii_hexdigit(),
            _ => return None,
        })
    }
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
                    if p[j] == '[' && j + 1 < p.len() && p[j + 1] == ':' {
                        if let Some(rel) = p[j + 2..].windows(2).position(|w| w == [':', ']']) {
                            let name: String = p[j + 2..j + 2 + rel].iter().collect();
                            if let Some(pred) = class_pred(&name) {
                                if pred(t[0]) { matched = true; }
                                j += rel + 4;
                                continue;
                            }
                        }
                    }
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
                        if m(rest, t) { return true; }
                        for a in &alts {
                            for i in 1..=t.len() {
                                if m(a, &t[..i]) && m(p, &t[i..]) { return true; }
                            }
                        }
                        false
                    }
                    '+' => {
                        for a in &alts {
                            for i in 1..=t.len() {
                                if m(a, &t[..i]) && m(p, &t[i..]) { return true; }
                            }
                        }
                        false
                    }
                    _ => {
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

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV___sh2_casesubj_1_.with(|v| *v.borrow_mut() = format!("{}", { let __sh_from_set: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); let __sh_to_set: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect(); std::env::var("var").unwrap_or_default().chars().map(|__sh_ch| match __sh_from_set.iter().position(|&__sh_from_ch| __sh_from_ch == __sh_ch) { Some(__sh_begin) => __sh_to_set[__sh_begin.min(__sh_to_set.len() - 1)], None => __sh_ch }).collect::<String>() }));
    if __sh_fnmatch("*[0-9]*", &__SHV___sh2_casesubj_1_.with(|v| v.borrow().clone())) {
        __SHV___sh2_casesubj_0_.with(|v| *v.borrow_mut() = format!("{}", std::env::var("var").unwrap_or_default().to_lowercase()));
        if __SHV___sh2_casesubj_0_.with(|v| v.borrow().clone()).contains("pattern") {
            print!("Double nested pattern\n");
        } else {
            print!("Single nested pattern\n");
        }
    } else {
        print!("No numbers\n");
    }
    std::process::ExitCode::SUCCESS
}