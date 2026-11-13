use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);
static __SH_BG: std::sync::Mutex<Vec<(u32, std::process::Child)>> = std::sync::Mutex::new(Vec::new());
static __SH_BGTHREADS: std::sync::Mutex<Vec<std::thread::JoinHandle<()>>> = std::sync::Mutex::new(Vec::new());
static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_BGPID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp0: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp1: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp2: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___sh2_tc0_: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_a: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_args: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_array: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_b: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_c: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_counter: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_d: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_e: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_f: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_flags: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_g: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_h: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_j: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_k: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_key: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_l: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_line: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_m: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_matrix: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_n: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_options: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_output: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_result: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_value: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_x: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_y: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_z: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_pow(a: i64, b: i64) -> i64 {
    let mut r: i64 = 1;
    let mut i: i64 = 0;
    while i < b { r = r.wrapping_mul(a); i += 1; }
    r
}

fn __sh_print_words(ws: &[Vec<String>], nl: bool, esc: bool) {
    // no echo in this program used -e, so `esc` is always false — the
    // unescape arm (and its __sh_echo_esc dependency) is compiled out.
    let _ = esc;
    // a word's `$((…))` arith expansion FAILED (an empty positional
    // makes the expanded text a syntax error) — bash suppresses the
    // WHOLE simple command, not just the word
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { return; }
    let mut s = String::new();
    let mut first = true;
    for w in ws {
        for x in w {
            if !first { s.push(' '); }
            first = false;
            s.push_str(x);
        }
    }
    if nl { s.push('\n'); }
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, s.as_bytes()); used = true; } });
    if used { return; }
    if let Some(b) = __SH_OUT.lock().unwrap().as_mut() {
        b.extend_from_slice(s.as_bytes());
        return;
    }
    print!("{}", s);
}

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

fn __sh_substr(s: &str, off: i64, len: i64) -> String {
    let ch: Vec<char> = s.chars().collect();
    let n = ch.len() as i64;
    let mut o = if off < 0 { n + off } else { off };
    if o < 0 { o = 0; }
    if o > n { return String::new(); }
    let mut l = if len == i64::MIN { n - o } else if len < 0 { n + len - o } else { len };
    if l < 0 { l = 0; }
    if o + l > n { l = n - o; }
    ch[o as usize..(o + l) as usize].iter().collect()
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

fn __sh_wait_all() {
    let hs = std::mem::take(&mut *__SH_BG.lock().unwrap());
    for (_, mut h) in hs { let _ = h.wait(); }
    let ths = std::mem::take(&mut *__SH_BGTHREADS.lock().unwrap());
    for th in ths { let _ = th.join(); }
}

fn __sh_bg(cmd: &str) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    match c.spawn() {
        Ok(ch) => {
            let pid = ch.id();
            __SH_BGPID.store(pid as i32, Ordering::SeqCst);
            __SH_BG.lock().unwrap().push((pid, ch));
        }
        Err(_) => {}
    }
}

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_result.with(|v| *v.borrow_mut() = vec![{ let __v = (((({ let __a = ((__SHV_a.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + __SHV_b.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0)) * (__SHV_c.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) - __SHV_d.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0))); let __b = { let __a = __SHV_e.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); let __b = __SHV_f.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); if __b == 0 { __SH_ARITH_ERR.store(true, Ordering::SeqCst); 0 } else { __a % __b } }; if __b == 0 { __SH_ARITH_ERR.store(true, Ordering::SeqCst); 0 } else { __a / __b } } + __sh_pow(__SHV_g.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0),__SHV_h.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0))) - (__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) << __SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0))) | ((__SHV_k.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) & __SHV_l.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0)) ^ (__SHV_m.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) | __SHV_n.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0))))).to_string(); let __e = __SH_ARITH_ERR.swap(false, Ordering::SeqCst); __SH_RC.store(if __e { 1 } else { 0 }, Ordering::SeqCst); if __e { String::new() } else { __v } }]);
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("0,0".to_string(), { let __v = (((__SHV_x.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + __SHV_y.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0)) * __SHV_z.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0))).to_string(); let __e = __SH_ARITH_ERR.swap(false, Ordering::SeqCst); __SH_RC.store(if __e { 1 } else { 0 }, Ordering::SeqCst); if __e { String::new() } else { __v } }); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("1,1".to_string(), __SHV_array.with(|v| v.borrow().get(0usize).cloned().unwrap_or_default())); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("2,2".to_string(), ["prefix"].join(" ")); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("3,3".to_string(), (__SHV_array.with(|v| v.borrow().len())).to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_output.with(|v| *v.borrow_mut() = {
    let __cap0_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap0_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap0_in = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![format!("Result: {}", {
    let __cap1_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap1_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap1_in = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![format!("Nested: {}", {
    let __cap2_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap2_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap2_in = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![format!("Deep: {}", {
    __SH_RC.store(0, Ordering::SeqCst);
    "Level 4".to_string()
})]], true, false);
    __SH_RC.store(0, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap2_in;
    let __cap2_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap2_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap2_outf);
    let mut __s = String::from_utf8_lossy(&__cap2_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
})]], true, false);
    __SH_RC.store(0, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap1_in;
    let __cap1_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap1_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap1_outf);
    let mut __s = String::from_utf8_lossy(&__cap1_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
})]], true, false);
    __SH_RC.store(0, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap0_in;
    let __cap0_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap0_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap0_outf);
    let mut __s = String::from_utf8_lossy(&__cap0_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    { let __w: &[Vec<String>] = {&[vec![{ let __v = std::env::var("var").unwrap_or_default(); if __v.is_empty() { format!("{}", { let __v = std::env::var("default").unwrap_or_default(); if __v.is_empty() { format!("{}", { let __v = std::env::var("fallback").unwrap_or_default(); if __v.is_empty() { "computed".to_string() } else { __v.to_string() } }) } else { __v.to_string() } }) } else { __v.to_string() } }]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    { let __w: &[Vec<String>] = {&[vec![__SHV_array.with(|v| v.borrow().get(0usize).cloned().unwrap_or_default())]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_ARITH_WORD_FAIL.store(false, Ordering::SeqCst);
    { let __ws = vec![{ __SH_ARITH_WORD_FAIL.store(true, Ordering::SeqCst); String::new() }]; let __wf = __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst); if !__wf { { let __w: &[Vec<String>] = {&[__ws]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } }; } __SH_RC.store(if __wf { 1 } else { 0 }, Ordering::SeqCst); }
    let mut __sh_pbuf11: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cat").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, "# This is a comment\n$(echo \"Command substitution\")\n${var:-default}\n$(( 1 + 2 * 3 ))\n".to_string().as_bytes()); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __sh_pbuf11 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("grep").arg("-v").arg("^#").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("sed").arg("s/^/  /").stdin(__x).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf11); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    __sh_bg(&format!("({}; {})", "'sleep' '1'".to_string(), "'echo' 'Starting'".to_string()));
    __sh_bg(&format!("({}; {})", "'sleep' '2'".to_string(), "'echo' 'Processing'".to_string()));
    __sh_wait_all();
    { let __w: &[Vec<String>] = {&[vec!["All done".to_string()]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    if (__sh_regex(&std::env::var("var").unwrap_or_default(), "^[0-9]+$", "") && std::env::var("var").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) > 0) && std::path::Path::new(&std::env::var("file").unwrap_or_default()).is_file() {
        if __sh_regex(&std::env::var("array[@]").unwrap_or_default(), "value", "") || __SHV_array.with(|v| v.borrow().len()) > 5 {
            if !__sh_capture_rc(&"echo \"$var\" | grep -q \"pattern\"").0.is_empty() {
                { let __w: &[Vec<String>] = {&[vec!["Deeply nested condition met".to_string()]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
            }
        }
    }
    let __sh_case15 = {
    let __cap3_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap3_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap3_in = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf13: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout14 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __w: &[Vec<String>] = {&[vec![std::env::var("var").unwrap_or_default()]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf13 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout14);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("tr").arg("[:upper:]").arg("[:lower:]").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf13); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    *__SH_STDIN.lock().unwrap() = __cap3_in;
    let __cap3_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap3_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap3_outf);
    let mut __s = String::from_utf8_lossy(&__cap3_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    if __sh_fnmatch("*[0-9]*", &__sh_case15) {
        let __sh_case16 = std::env::var("var").unwrap_or_default().to_lowercase();
        if { let __t: &str = {&__sh_case16}; __t.contains("pattern") } {
            print!("{}", "Double nested pattern\n");
        } else if true {
            print!("{}", "Single nested pattern\n");
        } else {
            __SH_RC.store(0, Ordering::SeqCst);
        }
    } else if true {
        print!("{}", "No numbers\n");
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    let __sh_arith_v17 = { let __v = 0; __SHV_i.with(|v| *v.borrow_mut() = __v.to_string()); __v };
    let __n18 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v17 };
    __SH_RC.store(if __n18 != 0 { 0 } else { 1 }, Ordering::SeqCst);
    while __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) < __SHV_array.with(|v| v.borrow().len() as i64) {
        let __sh_arith_v21 = { let __v = 0; __SHV_j.with(|v| *v.borrow_mut() = __v.to_string()); __v };
        let __n22 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v21 };
        __SH_RC.store(if __n22 != 0 { 0 } else { 1 }, Ordering::SeqCst);
        while __SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) < std::env::var("array[i]").unwrap_or_default().chars().count() as i64 {
            if __SHV_array.with(|v| v.borrow().get(__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) > std::env::var("threshold").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) {
                { let __val = (__SHV_result.with(|v| v.borrow().get(__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) + __SHV_array.with(|v| v.borrow().get(__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0)).to_string(); __SHV_result.with(|v| { let mut b = v.borrow_mut(); let i = __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) as usize; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __val; }); }
            }
            let __sh_arith_v25 = { let __o = __SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); __SHV_j.with(|v| *v.borrow_mut() = (__SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) +1).to_string()); __o };
            let __n26 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v25 };
            __SH_RC.store(if __n26 != 0 { 0 } else { 1 }, Ordering::SeqCst);
        }
        let __sh_arith_v27 = { let __o = __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); __SHV_i.with(|v| *v.borrow_mut() = (__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) +1).to_string()); __o };
        let __n28 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v27 };
        __SH_RC.store(if __n28 != 0 { 0 } else { 1 }, Ordering::SeqCst);
    }
    __SHV___ps_tmp0.with(|v| *v.borrow_mut() = {
    let (__c, __r) = { let __sc = std::process::Command::new("mktemp").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
});
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp0.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        __SH_PIPESTATUS.lock().unwrap().clear();
        __SH_RC.store(__sh_spawn(&format!("{} | {}", format!("{}{}", "'grep' '-v' '^#' ", { let __t: &str = {&std::env::var("input_file").unwrap_or_default()}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }), format!("{}{}", "'head' '-n' ", { let __t: &str = {&std::env::var("max_lines").unwrap_or_default()}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o })), Some(&[])), Ordering::SeqCst);
        __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV___ps_tmp0.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV___ps_tmp0.with(|v| v.borrow().clone())));
        while {
__SH_RC.store(0, Ordering::SeqCst);
if {
let (__ln, __any) = __sh_readline();
let __f = __sh_read_fields(&__ln, " \t\n", 1);
__SHV_line.with(|v| *v.borrow_mut() = __f.get(0).cloned().unwrap_or_default());
__SH_RC.store(if __any { 0 } else { 1 }, Ordering::SeqCst);
__any
} && { let __b = !__SHV_line.with(|v| v.borrow().clone()).is_empty(); __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b } {
    let _ = { let __v = (((__SHV_counter.with(|v| v.get()) < std::env::var("max_lines").unwrap_or_default().trim().parse::<i64>().unwrap_or(0)) as i64)); __SH_RC.store(if __v != 0 { 0 } else { 1 }, Ordering::SeqCst); __v };
}
__SH_RC.load(Ordering::SeqCst) == 0
} {
            if __sh_regex(&__SHV_line.with(|v| v.borrow().clone()), "^[[:space:]]*#", "") {
                continue;
            }
            let __sh_case32 = __SHV_line.with(|v| v.borrow().clone());
            if __sh_fnmatch("*\\$\\(*\\)*", &__sh_case32) {
                { let __w: &[Vec<String>] = {&[vec![format!("Contains command substitution: {}", __SHV_line.with(|v| v.borrow().clone()))]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
            } else if (__sh_fnmatch("*\\$\\{[^}]*\\}*", &__sh_case32)) {
                { let __w: &[Vec<String>] = {&[vec![format!("Contains parameter expansion: {}", __SHV_line.with(|v| v.borrow().clone()))]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
            } else if (__sh_fnmatch("*\\$\\(\\(*\\)\\)*", &__sh_case32)) {
                { let __w: &[Vec<String>] = {&[vec![format!("Contains arithmetic expansion: {}", __SHV_line.with(|v| v.borrow().clone()))]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
            } else {
            }
            let __sh_arith_v33 = { let __o = __SHV_counter.with(|v| v.get()); __SHV_counter.with(|v| v.set(__SHV_counter.with(|v| v.get()) +1)); __o };
            let __n34 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v33 };
            __SH_RC.store(if __n34 != 0 { 0 } else { 1 }, Ordering::SeqCst);
        }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp0.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
    { let __val = format!("{}", (std::env::var("var").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) + __SHV_array.with(|v| v.borrow().get(std::env::var("index").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0)).to_string()); __SHV_result.with(|v| { let mut b = v.borrow_mut(); let i = 0; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __val; }); }
    let mut __sh_pbuf35: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let mut __sh_pbuf37: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout38 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let mut __sh_pbuf39: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout40 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"Level 3\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    { let __t: &str = {"Level 4\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    { let __t: &str = {"Still level 4\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf39 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout40);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("Level").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf39); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    __sh_pbuf37 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout38);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("sed").arg("s/Level/Depth/").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf37); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    __sh_pbuf35 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("wc").arg("-l").stdin(std::process::Stdio::piped()).spawn() { Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf35); } __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1) }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    __SHV___ps_tmp1.with(|v| *v.borrow_mut() = {
    let __cap4_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap4_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap4_in = __SH_STDIN.lock().unwrap().take();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mktemp").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap4_in;
    let __cap4_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap4_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap4_outf);
    let mut __s = String::from_utf8_lossy(&__cap4_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::fs::File::create(__SHV___ps_tmp1.with(|v| v.borrow().clone())) { Ok(__rf) => match std::process::Command::new("sort").arg("file1.txt").stdout(__rf).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } }, Err(_) => 1 } }, Ordering::SeqCst);
    __SHV___ps_tmp2.with(|v| *v.borrow_mut() = {
    let __cap5_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap5_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap5_in = __SH_STDIN.lock().unwrap().take();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mktemp").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap5_in;
    let __cap5_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap5_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap5_outf);
    let mut __s = String::from_utf8_lossy(&__cap5_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::fs::File::create(__SHV___ps_tmp2.with(|v| v.borrow().clone())) { Ok(__rf) => match std::process::Command::new("sort").arg("file2.txt").stdout(__rf).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } }, Err(_) => 1 } }, Ordering::SeqCst);
    __SH_RC.store(__sh_spawn(&format!("{} 2>&{}", format!("{} > {}", format!("{}{}", "'diff' ", vec![{ let __t: &str = {&__SHV___ps_tmp1.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }, { let __t: &str = {&__SHV___ps_tmp2.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }].join(" ")), "'comparison.txt'".to_string()), 1), None), Ordering::SeqCst);
    let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp1.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp2.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
    if ((!std::env::var("var").unwrap_or_default().is_empty()) && std::path::Path::new(&std::env::var("file").unwrap_or_default()).is_file()) || std::path::Path::new(&std::env::var("dir").unwrap_or_default()).is_dir() && __sh_capture_rc(&"wc -l < \"$file\"").0.trim().parse::<i64>().unwrap_or(0) > 10 {
        { let __w: &[Vec<String>] = {&[vec!["Complex test passed".to_string()]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    }
    { let __w: &[Vec<String>] = {&[vec!["a1x a1y a1z a2x a2y a2z a3x a3y a3z b1x b1y b1z b2x b2y b2z b3x b3y b3z c1x c1y c1z c2x c2y c2z c3x c3y c3z".to_string()]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("tr").arg("[:upper:]").arg("[:lower:]").stdin(std::process::Stdio::piped()).spawn() { Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, format!("{}\n", {
    let __cap6_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap6_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap6_in = __SH_STDIN.lock().unwrap().take();
    { let __w: &[Vec<String>] = {&[vec![format!("UPPER: {}", std::env::var("var").unwrap_or_default().to_uppercase())]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store(0, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap6_in;
    let __cap6_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap6_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap6_outf);
    let mut __s = String::from_utf8_lossy(&__cap6_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
}).as_bytes()); } __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1) }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __sh_print_words(&[vec![{ let __a: Vec<String> = vec!["--long-option=value with spaces".to_string(), "--array-option".to_string(), "item1".to_string(), "item2".to_string(), "item3".to_string(), "--flag".to_string(), "positional argument".to_string(), { let __v = std::env::var("var").unwrap_or_default(); if __v.is_empty() { "default".to_string() } else { __v.to_string() } }, {
    __SH_RC.store(0, Ordering::SeqCst);
    "computed".to_string()
}]; complex_function(&__a) }]], true, false);
    let mut __sh_pbuf41: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __sh_sv43 = __SHV___sh2_tc0_.with(|v| v.borrow().clone());
    let __sh_sv44 = __SHV_array.with(|v| v.borrow().clone());
    let __sh_sv45 = __SHV_i.with(|v| v.borrow().clone());
    let __sh_sv46 = __SHV_result.with(|v| v.borrow().clone());
    __SHV___sh2_tc0_.with(|v| *v.borrow_mut() = {
    let __cap7_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap7_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap7_in = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf47: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout48 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __w: &[Vec<String>] = {&[vec![std::env::var("var").unwrap_or_default()]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf47 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout48);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("tr").arg("[:upper:]").arg("[:lower:]").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf47); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    *__SH_STDIN.lock().unwrap() = __cap7_in;
    let __cap7_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap7_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap7_outf);
    let mut __s = String::from_utf8_lossy(&__cap7_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    if __sh_regex(&__SHV___sh2_tc0_.with(|v| v.borrow().clone()), "^[a-z]+$", "") {
        let __sh_arith_v49 = { let __v = 0; __SHV_i.with(|v| *v.borrow_mut() = __v.to_string()); __v };
        let __n50 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v49 };
        __SH_RC.store(if __n50 != 0 { 0 } else { 1 }, Ordering::SeqCst);
        let mut __sh_while_ran51 = false;
        let mut __sh_while_last52 = 0;
        while __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) < __SHV_array.with(|v| v.borrow().len() as i64) {
            __sh_while_ran51 = true;
            if __SHV_array.with(|v| v.borrow().get(__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) > std::env::var("threshold").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) && std::path::Path::new(&std::env::var("files[i]").unwrap_or_default()).is_file() {
                { let __val = (__SHV_result.with(|v| v.borrow().get(__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) + __sh_capture_rc(&"(wc -l < \"${files[i]}\")").0.trim().parse::<i64>().unwrap_or(0)).to_string(); __SHV_result.with(|v| { let mut b = v.borrow_mut(); let i = __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) as usize; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __val; }); }
                __SH_RC.store(0, Ordering::SeqCst);
            } else {
                __SH_RC.store(0, Ordering::SeqCst);
            }
            let __sh_arith_v53 = { let __o = __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); __SHV_i.with(|v| *v.borrow_mut() = (__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) +1).to_string()); __o };
            let __n54 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v53 };
            __SH_RC.store(if __n54 != 0 { 0 } else { 1 }, Ordering::SeqCst);
            __sh_while_last52 = __SH_RC.load(Ordering::SeqCst);
        }
        if !__sh_while_ran51 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last52, Ordering::SeqCst); }
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    __SHV___sh2_tc0_.with(|v| *v.borrow_mut() = __sh_sv43);
    __SHV_array.with(|v| *v.borrow_mut() = __sh_sv44);
    __SHV_i.with(|v| *v.borrow_mut() = __sh_sv45);
    __SHV_result.with(|v| *v.borrow_mut() = __sh_sv46);
    __sh_pbuf41 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'sort' '-n'".to_string(), format!("{} > {}", "'tail' '-n' '5'".to_string(), "'final_result.txt'".to_string())), Some(&__sh_pbuf41)), Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}

fn complex_function(__argv: &[String]) -> String {
    __SHV_args.with(|v| *v.borrow_mut() = [__argv.to_vec()].concat());
    __SH_RC.store(0, Ordering::SeqCst);
    { let __words = Vec::<String>::new(); __SHV_options.with(|v| { let mut __m = v.borrow_mut(); __m.clear(); for __w in __words { if let Some(rest) = __w.strip_prefix('[') { if let Some(eq) = rest.find(']') { let __k = rest[..eq].to_string(); let __v = rest[eq + 2..].to_string(); __m.insert(__k, __v); } } } }); }
    __SHV_i.with(|v| *v.borrow_mut() = "0".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    let mut __sh_while_ran55 = false;
    let mut __sh_while_last56 = 0;
    while __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) < __SHV_args.with(|v| v.borrow().len() as i64) {
        __sh_while_ran55 = true;
        let __sh_case57 = { let __k: i64 = __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); let __n = __SHV_args.with(|v| v.borrow().len()) as i64; let __idx = if __k < 0 { __n + __k } else { __k }; __SHV_args.with(|v| v.borrow().get(__idx as usize).cloned().unwrap_or_default()) };
        if __sh_case57.starts_with("--") {
            __SHV_key.with(|v| *v.borrow_mut() = { let __sp = { let __k: i64 = __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); let __n = __SHV_args.with(|v| v.borrow().len()) as i64; let __idx = if __k < 0 { __n + __k } else { __k }; __SHV_args.with(|v| v.borrow().get(__idx as usize).cloned().unwrap_or_default()) }; __sp.strip_prefix("--").unwrap_or(&__sp).to_string() });
            __SH_RC.store(0, Ordering::SeqCst);
            __SHV_value.with(|v| *v.borrow_mut() = { let __k: i64 = (__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + 1); let __n = __SHV_args.with(|v| v.borrow().len()) as i64; let __idx = if __k < 0 { __n + __k } else { __k }; __SHV_args.with(|v| v.borrow().get(__idx as usize).cloned().unwrap_or_default()) });
            __SH_RC.store(0, Ordering::SeqCst);
            __SHV_options.with(|v| { v.borrow_mut().insert(format!("{}", __SHV_key.with(|v| v.borrow().clone())), __SHV_value.with(|v| v.borrow().clone())); });
            __SH_RC.store(0, Ordering::SeqCst);
            let __sh_arith_v58 = { __SHV_i.with(|v| *v.borrow_mut() = (__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + 2).to_string()); __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) };
            let __n59 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v58 };
            __SH_RC.store(if __n59 != 0 { 0 } else { 1 }, Ordering::SeqCst);
        } else if (__sh_case57.starts_with("-")) {
            __SHV_flags.with(|v| *v.borrow_mut() = { let __sp = { let __k: i64 = __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); let __n = __SHV_args.with(|v| v.borrow().len()) as i64; let __idx = if __k < 0 { __n + __k } else { __k }; __SHV_args.with(|v| v.borrow().get(__idx as usize).cloned().unwrap_or_default()) }; __sp.strip_prefix("-").unwrap_or(&__sp).to_string() });
            __SHV_j.with(|v| *v.borrow_mut() = "0".to_string());
            __SH_RC.store(0, Ordering::SeqCst);
            let mut __sh_while_ran60 = false;
            let mut __sh_while_last61 = 0;
            while __SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) < __SHV_flags.with(|v| v.borrow().clone()).chars().count() as i64 {
                __sh_while_ran60 = true;
                __SHV_options.with(|v| { v.borrow_mut().insert(format!("{}", __sh_substr(&__SHV_flags.with(|v| v.borrow().clone()), __SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0), 1)), "true".to_string()); });
                __SH_RC.store(0, Ordering::SeqCst);
                let __sh_arith_v62 = { let __o = __SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); __SHV_j.with(|v| *v.borrow_mut() = (__SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) +1).to_string()); __o };
                let __n63 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v62 };
                __SH_RC.store(if __n63 != 0 { 0 } else { 1 }, Ordering::SeqCst);
                __sh_while_last61 = __SH_RC.load(Ordering::SeqCst);
            }
            if !__sh_while_ran60 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last61, Ordering::SeqCst); }
            let __sh_arith_v64 = { let __o = __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); __SHV_i.with(|v| *v.borrow_mut() = (__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) +1).to_string()); __o };
            let __n65 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v64 };
            __SH_RC.store(if __n65 != 0 { 0 } else { 1 }, Ordering::SeqCst);
        } else if true {
            __SH_RC.store(0, Ordering::SeqCst);
            __sh_while_last56 = __SH_RC.load(Ordering::SeqCst);
            break;
        } else {
            __SH_RC.store(0, Ordering::SeqCst);
        }
        __sh_while_last56 = __SH_RC.load(Ordering::SeqCst);
    }
    if !__sh_while_ran55 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last56, Ordering::SeqCst); }
    let __v = format!("Processed {} options", __SHV_options.with(|v| v.borrow().len()).to_string()); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
}