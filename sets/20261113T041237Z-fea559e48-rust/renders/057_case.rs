use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
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

fn main() {
    print!("=== Basic Case Statement Example ===\n");
    let __sh_case0 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() };
    if (__sh_case0 == "start") {
        print!("Starting the service...\n");
    } else if ((__sh_case0 == "stop")) {
        print!("Stopping the service...\n");
    } else if ((__sh_case0 == "restart")) {
        print!("Restarting the service...\n");
    } else if true {
        println!("{}", format!("Usage: {} {{start|stop|restart}}", std::env::args().next().unwrap_or_default()));
        std::process::exit(1);
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    print!("=== Case Statement with Pattern Matching ===\n");
    let filename = { let __g = __SH_ARGV.lock().unwrap(); __g.get(1).cloned().unwrap_or_default() };
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_case1 = filename.clone();
    if __sh_case1.ends_with(".txt") {
        println!("{}", format!("Processing text file: {}", filename.clone()));
    } else if (__sh_case1.ends_with(".sh")) {
        println!("{}", format!("Processing shell script: {}", filename.clone()));
    } else if (__sh_case1.ends_with(".py")) {
        println!("{}", format!("Processing Python file: {}", filename.clone()));
    } else if true {
        println!("{}", format!("Unknown file type: {}", filename.clone()));
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    print!("=== Case Statement with Multiple Patterns ===\n");
    let __sh_case2 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(2).cloned().unwrap_or_default() };
    if (__sh_case2 == "help") || (__sh_case2 == "h") || (__sh_case2 == "-h") || (__sh_case2 == "--help") {
        print!("Help information:\n");
        print!("  start  - Start the service\n");
        print!("  stop   - Stop the service\n");
        print!("  status - Show service status\n");
    } else if ((__sh_case2 == "status") || (__sh_case2 == "s") || (__sh_case2 == "-s") || (__sh_case2 == "--status")) {
        print!("Service status: Running\n");
    } else if true {
        println!("{}", format!("Unknown option: {}", { let __g = __SH_ARGV.lock().unwrap(); __g.get(2).cloned().unwrap_or_default() }));
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    print!("=== Case Statement with Character Classes ===\n");
    let __sh_case3 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(3).cloned().unwrap_or_default() };
    if __sh_fnmatch("[0-9]", &__sh_case3) {
        println!("{}", format!("Single digit: {}", { let __g = __SH_ARGV.lock().unwrap(); __g.get(3).cloned().unwrap_or_default() }));
    } else if (__sh_fnmatch("[a-z]", &__sh_case3)) {
        println!("{}", format!("Lowercase letter: {}", { let __g = __SH_ARGV.lock().unwrap(); __g.get(3).cloned().unwrap_or_default() }));
    } else if (__sh_fnmatch("[A-Z]", &__sh_case3)) {
        println!("{}", format!("Uppercase letter: {}", { let __g = __SH_ARGV.lock().unwrap(); __g.get(3).cloned().unwrap_or_default() }));
    } else if (__sh_fnmatch("[0-9][0-9]", &__sh_case3)) {
        println!("{}", format!("Two digit number: {}", { let __g = __SH_ARGV.lock().unwrap(); __g.get(3).cloned().unwrap_or_default() }));
    } else if true {
        println!("{}", format!("Other character: {}", { let __g = __SH_ARGV.lock().unwrap(); __g.get(3).cloned().unwrap_or_default() }));
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    print!("=== Case Statement with Default Action ===\n");
    let __sh_case4 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(4).cloned().unwrap_or_default() };
    if (__sh_case4 == "red") {
        print!("Color is red\n");
    } else if ((__sh_case4 == "green")) {
        print!("Color is green\n");
    } else if ((__sh_case4 == "blue")) {
        print!("Color is blue\n");
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    print!("=== Case Statement with Commands ===\n");
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_case5 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(5).cloned().unwrap_or_default() };
    if (__sh_case5 == "ls") {
        __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("ls").arg("-l").arg("-a").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    } else if ((__sh_case5 == "date")) {
        __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("date").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    } else if ((__sh_case5 == "pwd")) {
        println!("{}", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default());
    } else if ((__sh_case5 == "whoami")) {
        __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("whoami").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    } else if true {
        print!("Available commands: ls, date, pwd, whoami\n");
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}