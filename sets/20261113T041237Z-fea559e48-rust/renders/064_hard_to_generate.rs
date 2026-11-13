use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());
static __SH_TRAPS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_DEBUG: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_LOG_LEVEL: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp0: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp1: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp2: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp3: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp4: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp5: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_basename: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_config: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_extension: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_first_half: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_gid: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_home: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_info: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_last_half: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_middle: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_numbers: std::cell::RefCell<Vec<i64>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_pass: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_result: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_shell: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_uid: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_user: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    c.stdin(std::process::Stdio::inherit());
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let mut s = String::from_utf8_lossy(&out).to_string();
    while s.ends_with('\n') { s.pop(); }
    (s, rc)
}

fn __sh_spawn(cmd: &str, input: Option<&[u8]>) -> i32 {
    // a shell-out inside a native stage inherits the stage's stdin
    // (`echo x | grep -f <(…)` — the grep reads the echo's buffer)
    let mut input = input.map(|d| d.to_vec());
    let stdin_path = __SH_STDIN_PATH.lock().unwrap().clone();
    if input.is_none() && stdin_path.is_none() {
        if let Some(mut r) = __SH_STDIN.lock().unwrap().take() {
            let mut d = Vec::new();
            let _ = std::io::Read::read_to_end(&mut r, &mut d);
            input = Some(d);
        }
    }
    let want_out = __SH_OUT.lock().unwrap().is_some() || __SH_OUTFILE_TL.with(|v| v.borrow().is_some());
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    if !want_out {
        // the child inherits fd 1 raw — flush Rust's buffered stdout
        // first so prints BEFORE the child hit the pipe first
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
    if want_out { c.stdout(std::process::Stdio::piped()); }
    if input.is_none() {
        if let Some(p) = &stdin_path {
            // the child must see the REAL device fd (`tty < /dev/pts/5`
            // needs isatty, which a byte pipe lacks)
            if let Ok(f) = std::fs::File::open(p) {
                c.stdin(std::process::Stdio::from(f));
            }
        }
    }
    if input.is_some() { c.stdin(std::process::Stdio::piped()); }
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return 1 };
    if let Some(data) = input {
        if let Some(mut si) = ch.stdin.take() { let _ = std::io::Write::write_all(&mut si, &data); }
    }
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    if !out.is_empty() {
        let mut used = false;
        __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &out); used = true; } });
        if !used {
            if let Some(b) = __SH_OUT.lock().unwrap().as_mut() {
                b.extend_from_slice(&out);
            } else {
                let _ = std::io::Write::write_all(&mut std::io::stdout(), &out);
            }
        }
    }
    rc
}

fn __sh_readline() -> (String, bool) {
    let mut buf: Vec<u8> = Vec::new();
    let mut one = [0u8; 1];
    let mut any = false;
    let src: Box<dyn std::io::Read + Send> = match __SH_STDIN.lock().unwrap().take() {
        Some(r) => r,
        None => Box::new(std::io::stdin()),
    };
    let mut r = src;
    loop {
        match r.read(&mut one) {
            Ok(0) => break,
            Ok(_) => {
                if one[0] == b'\n' { any = true; break; }
                buf.push(one[0]);
                any = true;
            }
            Err(_) => break,
        }
    }
    *__SH_STDIN.lock().unwrap() = Some(r);
    (String::from_utf8_lossy(&buf).to_string(), any)
}

fn __sh_read_fields(line: &str, ifs: &str, n: usize) -> Vec<String> {
    if n <= 1 || ifs.is_empty() { return vec![line.to_string()]; }
    let ws = ifs.chars().all(|c| c == ' ' || c == '\t' || c == '\n');
    let ch: Vec<char> = line.chars().collect();
    let is_sep = |c: char| ifs.contains(c);
    let mut fields: Vec<String> = Vec::new();
    let mut i = 0;
    while i < ch.len() && is_sep(ch[i]) { i += 1; }
    while fields.len() < n - 1 && i < ch.len() {
        let start = i;
        while i < ch.len() && !is_sep(ch[i]) { i += 1; }
        fields.push(ch[start..i].iter().collect());
        if i >= ch.len() { break; }
        i += 1;
        if ws { while i < ch.len() && is_sep(ch[i]) { i += 1; } }
    }
    let rest: String = ch[i.min(ch.len())..].iter().collect();
    fields.push(rest);
    fields
}

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

fn __sh_regex(text: &str, pat: &str, flags: &str) -> bool {
    let cmd = if flags.contains('i') {
        "shopt -s nocasematch; [[ $1 =~ $2 ]]"
    } else {
        "[[ $1 =~ $2 ]]"
    };
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd).arg("sh").arg(text).arg(pat);
    c.status().map(|s| s.success()).unwrap_or(false)
}

fn __sh_capture(cmd: &str) -> String {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdin(std::process::Stdio::inherit());
    match c.output() {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Err(_) => String::new(),
    }
}

fn __sh_run_traps() {
    let hs = std::mem::take(&mut *__SH_TRAPS.lock().unwrap());
    for h in hs { let _ = __sh_spawn(&h, None); }
}

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    __SHV___ps_tmp1.with(|v| *v.borrow_mut() = {
    let (__c, __r) = { let __sc = std::process::Command::new("mktemp").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
});
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp1.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        __SHV___ps_tmp0.with(|v| *v.borrow_mut() = {
    let __cap0_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap0_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap0_in = __SH_STDIN.lock().unwrap().take();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mktemp").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap0_in;
    let __cap0_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap0_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap0_outf);
    let mut __s = String::from_utf8_lossy(&__cap0_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
        {
            let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
            __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp0.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
            __SH_PIPESTATUS.lock().unwrap().clear();
            __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("grep").arg("-v").arg("^#").arg("/etc/passwd").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("cut").arg("-d").arg(":").arg("-f").arg("1").stdin(__x).stdout(std::process::Stdio::piped()).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &[]); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
            __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
            __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
        }
        __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("sort").arg(__SHV___ps_tmp0.with(|v| v.borrow().clone())).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
        let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp0.with(|v| v.borrow().clone())).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    __SHV___ps_tmp3.with(|v| *v.borrow_mut() = {
    let __cap1_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap1_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap1_in = __SH_STDIN.lock().unwrap().take();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mktemp").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap1_in;
    let __cap1_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap1_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap1_outf);
    let mut __s = String::from_utf8_lossy(&__cap1_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp3.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        __SHV___ps_tmp2.with(|v| *v.borrow_mut() = {
    let __cap2_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap2_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap2_in = __SH_STDIN.lock().unwrap().take();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mktemp").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap2_in;
    let __cap2_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap2_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap2_outf);
    let mut __s = String::from_utf8_lossy(&__cap2_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
        {
            let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
            __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp2.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
            __SH_PIPESTATUS.lock().unwrap().clear();
            __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("grep").arg("-v").arg("^#").arg("/etc/group").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("cut").arg("-d").arg(":").arg("-f").arg("1").stdin(__x).stdout(std::process::Stdio::piped()).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &[]); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
            __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
            __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
        }
        __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("sort").arg(__SHV___ps_tmp2.with(|v| v.borrow().clone())).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
        let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp2.with(|v| v.borrow().clone())).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("diff").arg(__SHV___ps_tmp1.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp3.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp1.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp3.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
    println!("{}", "Files:  file_a_1..10.txt file_a_1..10.log file_a_1..10.dat file_a_20.txt file_a_20.log file_a_20.dat file_a_30..40.txt file_a_30..40.log file_a_30..40.dat file_b_1..10.txt file_b_1..10.log file_b_1..10.dat file_b_20.txt file_b_20.log file_b_20.dat file_b_30..40.txt file_b_30..40.log file_b_30..40.dat file_c_1..10.txt file_c_1..10.log file_c_1..10.dat file_c_20.txt file_c_20.log file_c_20.dat file_c_30..40.txt file_c_30..40.log file_c_30..40.dat file_d_1..10.txt file_d_1..10.log file_d_1..10.dat file_d_20.txt file_d_20.log file_d_20.dat file_d_30..40.txt file_d_30..40.log file_d_30..40.dat file_e_1..10.txt file_e_1..10.log file_e_1..10.dat file_e_20.txt file_e_20.log file_e_20.dat file_e_30..40.txt file_e_30..40.log file_e_30..40.dat file_f_1..10.txt file_f_1..10.log file_f_1..10.dat file_f_20.txt file_f_20.log file_f_20.dat file_f_30..40.txt file_f_30..40.log file_f_30..40.dat file_g_1..10.txt file_g_1..10.log file_g_1..10.dat file_g_20.txt file_g_20.log file_g_20.dat file_g_30..40.txt file_g_30..40.log file_g_30..40.dat file_h_1..10.txt file_h_1..10.log file_h_1..10.dat file_h_20.txt file_h_20.log file_h_20.dat file_h_30..40.txt file_h_30..40.log file_h_30..40.dat file_i_1..10.txt file_i_1..10.log file_i_1..10.dat file_i_20.txt file_i_20.log file_i_20.dat file_i_30..40.txt file_i_30..40.log file_i_30..40.dat file_j_1..10.txt file_j_1..10.log file_j_1..10.dat file_j_20.txt file_j_20.log file_j_20.dat file_j_30..40.txt file_j_30..40.log file_j_30..40.dat file_k_1..10.txt file_k_1..10.log file_k_1..10.dat file_k_20.txt file_k_20.log file_k_20.dat file_k_30..40.txt file_k_30..40.log file_k_30..40.dat file_l_1..10.txt file_l_1..10.log file_l_1..10.dat file_l_20.txt file_l_20.log file_l_20.dat file_l_30..40.txt file_l_30..40.log file_l_30..40.dat file_m_1..10.txt file_m_1..10.log file_m_1..10.dat file_m_20.txt file_m_20.log file_m_20.dat file_m_30..40.txt file_m_30..40.log file_m_30..40.dat file_n_1..10.txt file_n_1..10.log file_n_1..10.dat file_n_20.txt file_n_20.log file_n_20.dat file_n_30..40.txt file_n_30..40.log file_n_30..40.dat file_o_1..10.txt file_o_1..10.log file_o_1..10.dat file_o_20.txt file_o_20.log file_o_20.dat file_o_30..40.txt file_o_30..40.log file_o_30..40.dat file_p_1..10.txt file_p_1..10.log file_p_1..10.dat file_p_20.txt file_p_20.log file_p_20.dat file_p_30..40.txt file_p_30..40.log file_p_30..40.dat file_q_1..10.txt file_q_1..10.log file_q_1..10.dat file_q_20.txt file_q_20.log file_q_20.dat file_q_30..40.txt file_q_30..40.log file_q_30..40.dat file_r_1..10.txt file_r_1..10.log file_r_1..10.dat file_r_20.txt file_r_20.log file_r_20.dat file_r_30..40.txt file_r_30..40.log file_r_30..40.dat file_s_1..10.txt file_s_1..10.log file_s_1..10.dat file_s_20.txt file_s_20.log file_s_20.dat file_s_30..40.txt file_s_30..40.log file_s_30..40.dat file_t_1..10.txt file_t_1..10.log file_t_1..10.dat file_t_20.txt file_t_20.log file_t_20.dat file_t_30..40.txt file_t_30..40.log file_t_30..40.dat file_u_1..10.txt file_u_1..10.log file_u_1..10.dat file_u_20.txt file_u_20.log file_u_20.dat file_u_30..40.txt file_u_30..40.log file_u_30..40.dat file_v_1..10.txt file_v_1..10.log file_v_1..10.dat file_v_20.txt file_v_20.log file_v_20.dat file_v_30..40.txt file_v_30..40.log file_v_30..40.dat file_w_1..10.txt file_w_1..10.log file_w_1..10.dat file_w_20.txt file_w_20.log file_w_20.dat file_w_30..40.txt file_w_30..40.log file_w_30..40.dat file_x_1..10.txt file_x_1..10.log file_x_1..10.dat file_x_20.txt file_x_20.log file_x_20.dat file_x_30..40.txt file_x_30..40.log file_x_30..40.dat file_y_1..10.txt file_y_1..10.log file_y_1..10.dat file_y_20.txt file_y_20.log file_y_20.dat file_y_30..40.txt file_y_30..40.log file_y_30..40.dat file_z_1..10.txt file_z_1..10.log file_z_1..10.dat file_z_20.txt file_z_20.log file_z_20.dat file_z_30..40.txt file_z_30..40.log file_z_30..40.dat".to_string());
    let name = "John Doe";
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("Hello {}", "John_Doe".to_string()));
    println!("{}", format!("Length: {}", "8".to_string()));
    println!("{}", format!("First: {}", "John".to_string()));
    println!("{}", format!("Last: {}", name.chars().skip(name.chars().count().saturating_sub(3)).collect::<String>()));
    let _ = String::new();
    let _ = String::new();
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_case2 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() };
    if __sh_fnmatch("[a-z]*", &__sh_case2) {
        print!("{}", "Lowercase start\n");
    } else if (__sh_fnmatch("[A-Z]*", &__sh_case2)) {
        print!("{}", "Uppercase start\n");
    } else if (__sh_fnmatch("[0-9]*", &__sh_case2)) {
        print!("{}", "Number start\n");
    } else if (__sh_fnmatch("?", &__sh_case2)) {
        print!("{}", "Single character\n");
    } else if true {
        print!("{}", "Something else\n");
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    #[allow(unused_assignments)] let mut i = { let __v = (2).to_string(); let __e = __SH_ARITH_ERR.swap(false, Ordering::SeqCst); __SH_RC.store(if __e { 1 } else { 0 }, Ordering::SeqCst); if __e { String::new() } else { __v } };
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_arith_v3 = ({ let __o = i.clone().trim().parse::<i64>().unwrap_or(0); i = (i.clone().trim().parse::<i64>().unwrap_or(0) +1).to_string(); __o } + { i = (i.clone().trim().parse::<i64>().unwrap_or(0) +1).to_string(); i.clone().trim().parse::<i64>().unwrap_or(0) }).to_string();
    let j = __sh_arith_v3.clone();
    let _ = { let __v = __sh_arith_v3; let __n = __v.trim().parse::<i64>().unwrap_or(0); __SH_RC.store(if __n != 0 { 0 } else { 1 }, Ordering::SeqCst); __v };
    println!("{}", format!("i={}, j={}", i.clone(), j.clone()));
    __SHV_config.with(|v| { v.borrow_mut().insert("user".to_string(), "admin".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_config.with(|v| { v.borrow_mut().insert("host".to_string(), "localhost".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_config.with(|v| { v.borrow_mut().insert("port".to_string(), "8080".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'cat'".to_string(), "'config.txt'".to_string()), Some("User: $USER\nHost: ${HOSTNAME:-localhost}\nPath: $PWD\n".to_string().as_bytes())), Ordering::SeqCst);
    __SHV___ps_tmp4.with(|v| *v.borrow_mut() = {
    let (__c, __r) = { let __sc = std::process::Command::new("mktemp").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
});
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp4.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        __SH_PIPESTATUS.lock().unwrap().clear();
        __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("cut").arg("-d").arg(":").arg("-f").arg("1").arg("/etc/passwd").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("sort").stdin(__x).stdout(std::process::Stdio::piped()).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &[]); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
        __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    __SHV___ps_tmp5.with(|v| *v.borrow_mut() = {
    let __cap3_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap3_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap3_in = __SH_STDIN.lock().unwrap().take();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mktemp").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap3_in;
    let __cap3_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap3_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap3_outf);
    let mut __s = String::from_utf8_lossy(&__cap3_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp5.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        __SH_PIPESTATUS.lock().unwrap().clear();
        __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("cut").arg("-d").arg(":").arg("-f").arg("3").arg("/etc/passwd").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("sort").arg("-n").stdin(__x).stdout(std::process::Stdio::piped()).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &[]); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
        __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", format!("{}{}", "'paste' ", vec![{ let __t: &str = {&__SHV___ps_tmp4.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }, { let __t: &str = {&__SHV___ps_tmp5.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }].join(" ")), "'head' '-10'".to_string()), Some(&[])), Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp4.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp5.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
    if __sh_regex(&{ let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() }, "^[0-9]+$", "") && ({ let __g = __SH_ARGV.lock().unwrap(); __g.get(1).cloned().unwrap_or_default() }.to_lowercase() == "test".to_string().to_lowercase()) || ({ let __g = __SH_ARGV.lock().unwrap(); __g.get(1).cloned().unwrap_or_default() }.to_lowercase() == "debug".to_string().to_lowercase()) {
        println!("{}", "Valid input".to_string());
    }
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mkdir").arg("-p").arg("project/src/main/java").arg("project/src/main/resources").arg("project/src/test/java").arg("project/src/test/resources").arg("project/docs/api").arg("project/docs/user").arg("project/build/classes").arg("project/build/lib").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    let filename = "my_file.txt";
    __SHV_basename.with(|v| *v.borrow_mut() = { let __v = {&filename}.to_string(); match __v.rfind(".") { Some(__i) => __v[..__i].to_string(), None => __v } });
    __SHV_extension.with(|v| *v.borrow_mut() = { let __v = {&filename}.to_string(); match __v.rfind(".") { Some(__i) => __v[__i + ".".len()..].to_string(), None => __v } });
    #[allow(unused_assignments)] let mut uppercase = "MY_FILE.TXT".to_string();
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_result.with(|v| *v.borrow_mut() = {
    let __cap4_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap4_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap4_in = __SH_STDIN.lock().unwrap().take();
    { let __w: &[Vec<String>] = {&[vec![((__sh_capture_rc(&"(wc -l < /etc/passwd)").0.trim().parse::<i64>().unwrap_or(0) + __sh_capture_rc(&"(wc -l < /etc/group)").0.trim().parse::<i64>().unwrap_or(0))).to_string()]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store(0, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap4_in;
    let __cap4_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap4_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap4_outf);
    let mut __s = String::from_utf8_lossy(&__cap4_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", format!("{} | {}", format!("{} | {}", "'grep' '-v' '^#' '/etc/passwd'".to_string(), "'cut' '-d' ':' '-f' '1,3'".to_string()), "'sort' '-t' ':' '-k' '2' '-n'".to_string()), format!("{} 2> {}", format!("{} > {}", "'head' '-5'".to_string(), "'users.txt'".to_string()), "'errors.log'".to_string())), Some(&[])), Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/etc/passwd").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/etc/passwd"));
        while {
let (__ln, __any) = __sh_readline();
let __f = __sh_read_fields(&__ln, ":", 7);
__SHV_user.with(|v| *v.borrow_mut() = __f.get(0).cloned().unwrap_or_default());
__SHV_pass.with(|v| *v.borrow_mut() = __f.get(1).cloned().unwrap_or_default());
__SHV_uid.with(|v| *v.borrow_mut() = __f.get(2).cloned().unwrap_or_default());
__SHV_gid.with(|v| *v.borrow_mut() = __f.get(3).cloned().unwrap_or_default());
__SHV_info.with(|v| *v.borrow_mut() = __f.get(4).cloned().unwrap_or_default());
__SHV_home.with(|v| *v.borrow_mut() = __f.get(5).cloned().unwrap_or_default());
__SHV_shell.with(|v| *v.borrow_mut() = __f.get(6).cloned().unwrap_or_default());
__SH_RC.store(if __any { 0 } else { 1 }, Ordering::SeqCst);
__any
} {
            if (__SHV_uid.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) > 1000) && !(__SHV_shell.with(|v| v.borrow().clone()).to_lowercase() == "/bin/false".to_string().to_lowercase()) {
                if __sh_regex(&__SHV_home.with(|v| v.borrow().clone()), "^/home/", "") {
                    println!("{}", format!("User: {} (UID: {}) - {}", __SHV_user.with(|v| v.borrow().clone()), __SHV_uid.with(|v| v.borrow().clone()), __SHV_home.with(|v| v.borrow().clone())));
                }
            }
        }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    __SHV_numbers.with(|v| *v.borrow_mut() = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_middle.with(|v| *v.borrow_mut() = vec![{ let __v = __SHV_numbers.with(|v| v.borrow().iter().map(|n| n.to_string()).collect::<Vec<String>>()); let __s = 3usize.min(__v.len()); let __e = (__s + 4usize).min(__v.len()); __v[__s..__e].join(" ") }]);
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_first_half.with(|v| *v.borrow_mut() = vec![{ let __v = __SHV_numbers.with(|v| v.borrow().iter().map(|n| n.to_string()).collect::<Vec<String>>()); let __s = 0usize.min(__v.len()); let __e = (__s + 5usize).min(__v.len()); __v[__s..__e].join(" ") }]);
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_last_half.with(|v| *v.borrow_mut() = vec![{ let __v = __SHV_numbers.with(|v| v.borrow().iter().map(|n| n.to_string()).collect::<Vec<String>>()); let __s = 5usize.min(__v.len()); __v[__s..].join(" ") }]);
    __SH_RC.store(0, Ordering::SeqCst);
    for file in ["\x01SH2GLOB\x01*.txt", "\x01SH2GLOB\x01*.log", "\x01SH2GLOB\x01*.dat"] {
        let __sh_case11 = file;
        if { let __t: &str = {&__sh_case11}; __t.ends_with(".txt") } || { let __t: &str = {&__sh_case11}; __t.ends_with(".log") } {
            println!("{}", format!("Text file: {}", file));
        } else if ({ let __t: &str = {&__sh_case11}; __t.ends_with(".dat") }) {
            println!("{}", format!("Data file: {}", file));
        } else if true {
            println!("{}", format!("Other file: {}", file));
        } else {
        }
    }
    let __sh_sv12 = __SHV_DEBUG.with(|v| v.borrow().clone());
    let __sh_sv13 = __SHV_LOG_LEVEL.with(|v| v.borrow().clone());
    __SHV_DEBUG.with(|v| *v.borrow_mut() = "1".to_string());
    std::env::set_var("DEBUG", "1");
    __SHV_LOG_LEVEL.with(|v| *v.borrow_mut() = "verbose".to_string());
    std::env::set_var("LOG_LEVEL", "verbose");
    let __sh_sv14 = __SHV_DEBUG.with(|v| v.borrow().clone());
    __SHV_DEBUG.with(|v| *v.borrow_mut() = String::new());
    println!("{}", format!("Inner: LOG_LEVEL={}, DEBUG={}", __SHV_LOG_LEVEL.with(|v| v.borrow().clone()), { let __v = __SHV_DEBUG.with(|v| v.borrow().clone()); if __v.is_empty() { "unset".to_string() } else { __v.to_string() } }));
    __SHV_DEBUG.with(|v| *v.borrow_mut() = __sh_sv14);
    println!("{}", format!("Outer: LOG_LEVEL={}, DEBUG={}", __SHV_LOG_LEVEL.with(|v| v.borrow().clone()), __SHV_DEBUG.with(|v| v.borrow().clone())));
    __SHV_DEBUG.with(|v| *v.borrow_mut() = __sh_sv12);
    __SHV_LOG_LEVEL.with(|v| *v.borrow_mut() = __sh_sv13);
    let message = format!("Hello {} from {}", { let __v = std::env::var("USER").unwrap_or_default(); if __v.is_empty() { "guest".to_string() } else { __v.to_string() } }, { let __v = __sh_capture("hostname"); if __v.is_empty() { "localhost".to_string() } else { __v.to_string() } });
    println!("{}", message.clone());
    __SH_TRAPS.lock().unwrap().push("echo \"Cleaning up...\"; rm -f /tmp/064_hard_temp_*".to_string());
    let input = { let __v = { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() }; if __v.is_empty() { "default_value".to_string() } else { __v.to_string() } };
    let sanitized = __sh_replace(&input.clone(), "[^a-zA-Z0-9]", "_", true);
    uppercase = sanitized.clone().to_uppercase();
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("Input: '{}' -> Sanitized: '{}' -> Uppercase: '{}'", input.clone(), sanitized.clone(), uppercase.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    if !({ let __b = std::path::Path::new(&{ let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() }).is_file(); __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b } && {
print!("{}", "File exists\n");
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
}) {
        print!("{}", "File not found\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    if !(({ let __b = std::path::Path::new(&{ let __g = __SH_ARGV.lock().unwrap(); __g.get(1).cloned().unwrap_or_default() }).is_dir(); __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b } && { let __d = { let __g = __SH_ARGV.lock().unwrap(); __g.get(1).cloned().unwrap_or_default() }; let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() }) && {
println!("{}", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default());
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
}) {
        print!("{}", "Directory not accessible\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    println!("{}", "Script completed successfully!".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_final = __SH_RC.load(Ordering::SeqCst);
    __sh_run_traps();
    std::process::exit(__sh_final);
}