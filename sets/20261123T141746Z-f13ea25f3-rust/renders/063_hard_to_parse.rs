use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);
static __SH_BG: std::sync::Mutex<Vec<(u32, std::process::Child)>> = std::sync::Mutex::new(Vec::new());
static __SH_BGTHREADS: std::sync::Mutex<Vec<std::thread::JoinHandle<()>>> = std::sync::Mutex::new(Vec::new());
static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp0: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp1: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp2: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___sh2_casesubj_0_: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___sh2_casesubj_1_: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___sh2_casesubj_2_: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
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

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn __sh_wq(ws: &[String]) -> String {
    ws.iter().map(|w| __sh_q(w)).collect::<Vec<_>>().join(" ")
}

fn __sh_pow(a: i64, b: i64) -> i64 {
    a.wrapping_pow(b.clamp(0, u32::MAX as i64) as u32)
}

fn __sh_print_words(ws: &[Vec<String>], nl: bool, _esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    if nl { s.push('\n'); }
    __sh_sink(s.as_bytes());
}

fn __sh_puts(s: &str) {
    __sh_sink(s.as_bytes());
}

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    c.stdin(std::process::Stdio::inherit());
    let Ok(mut ch) = c.spawn() else { return (String::new(), 1) };
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
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
    if want_out { c.stdout(std::process::Stdio::piped()); }
    if input.is_none() {
        if let Some(p) = &stdin_path {
            if let Ok(f) = std::fs::File::open(p) {
                c.stdin(std::process::Stdio::from(f));
            }
        }
    }
    if input.is_some() { c.stdin(std::process::Stdio::piped()); }
    let Ok(mut ch) = c.spawn() else { return 1 };
    if let Some(data) = input {
        if let Some(mut si) = ch.stdin.take() { let _ = std::io::Write::write_all(&mut si, &data); }
    }
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    if !out.is_empty() { __sh_sink(&out); }
    rc
}

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

fn __sh_regex(text: &str, pat: &str, flags: &str) -> Option<bool> {
    let pat = if flags.contains('i') { format!("(?i){}", pat) } else { pat.to_string() };
    match regex::Regex::new(&pat) {
        Ok(re) => Some(re.is_match(text)),
        Err(_) => None,
    }
}

fn __sh_wait_all() {
    let hs = std::mem::take(&mut *__SH_BG.lock().unwrap());
    for (_, mut h) in hs { let _ = h.wait(); }
    let ths = std::mem::take(&mut *__SH_BGTHREADS.lock().unwrap());
    for th in ths { let _ = th.join(); }
}

fn __sh_div(a: i64, b: i64) -> i64 {
    if b == 0 { __SH_ARITH_ERR.store(true, Ordering::Relaxed); 0 } else { a / b }
}

fn __sh_mod(a: i64, b: i64) -> i64 {
    if b == 0 { __SH_ARITH_ERR.store(true, Ordering::Relaxed); 0 } else { a % b }
}

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
}

fn __sh_mktemp(dir: bool, tmpl: &str) -> String {
    let t = if tmpl.is_empty() {
        let d = std::env::var("TMPDIR").ok().filter(|d| !d.is_empty()).unwrap_or_else(|| "/tmp".to_string());
        format!("{}/tmp.XXXXXXXXXX", d.trim_end_matches('/'))
    } else {
        tmpl.to_string()
    };
    let base = t.trim_end_matches('X').to_string();
    let nx = t.len() - base.len();
    let nx = if nx == 0 { 10 } else { nx };
    let cs: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    for n in 0..100000u64 {
        let seed = (std::process::id() as u64).wrapping_mul(1_000_003)
            ^ n.wrapping_mul(7_919)
            ^ std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos() as u64)
                .unwrap_or(0);
        let mut x = seed | 1;
        let mut s = base.clone();
        for _ in 0..nx { s.push(cs[(x % 62) as usize] as char); x /= 62; }
        let ok = if dir {
            std::fs::create_dir(&s).is_ok()
        } else {
            std::fs::OpenOptions::new().write(true).create_new(true).open(&s).is_ok()
        };
        if ok { return s; }
    }
    String::new()
}

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_result.with(|v| *v.borrow_mut() = vec![{ let __sh_value = ((((__sh_div((__SHV_a.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) + __SHV_b.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0))) * (__SHV_c.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) - __SHV_d.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0))),__sh_mod(__SHV_e.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)),__SHV_f.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)))) + __sh_pow(__SHV_g.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)),__SHV_h.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)))) - (__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) << __SHV_j.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)))) | ((__SHV_k.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) & __SHV_l.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0))) ^ (__SHV_m.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) | __SHV_n.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)))))).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } }]);
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("0,0".to_string(), { let __sh_value = (((__SHV_x.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) + __SHV_y.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0))) * __SHV_z.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)))).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } }); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("1,1".to_string(), __SHV_array.with(|v| v.borrow().get(0 as usize).cloned().unwrap_or_default())); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("2,2".to_string(), ["prefix"].join(" ")); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("3,3".to_string(), __SHV_array.with(|v| v.borrow().len().to_string())); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_output.with(|v| *v.borrow_mut() = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![format!("Result: {}", {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![format!("Nested: {}", {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![format!("Deep: {}", {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_puts("Level 4\n");
    __SH_RC.store(0, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
})]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
})]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
})]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
});
    println!("{}", { let __sh_value = std::env::var("var").unwrap_or_default(); if __sh_value.is_empty() { format!("{}", { let __sh_value = std::env::var("default").unwrap_or_default(); if __sh_value.is_empty() { format!("{}", { let __sh_value = std::env::var("fallback").unwrap_or_default(); if __sh_value.is_empty() { "computed".to_string() } else { __sh_value.to_string() } }) } else { __sh_value.to_string() } }) } else { __sh_value.to_string() } });
    println!("{}", { let __sh_idx = 0; let __sh_num = __SHV_array.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_array.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } });
    __SH_ARITH_WORD_FAIL.store(false, Ordering::Relaxed);
    { let __sh_wordlist = vec![{ __SH_ARITH_WORD_FAIL.store(true, Ordering::Relaxed); String::new() }]; let __sh_wordfail = __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed); if !__sh_wordfail { __sh_print_words(&[__sh_wordlist], true, false); } __SH_RC.store(if __sh_wordfail { 1 } else { 0 }, Ordering::Relaxed); }
    let mut __sh_pbuf10: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout11 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __sh_c = "# This is a comment\n$(echo \"Command substitution\")\n${var:-default}\n$(( 1 + 2 * 3 ))\n".to_string(); __sh_sink(__sh_c.as_bytes()); __SH_RC.store(0, Ordering::Relaxed); }
    __sh_pbuf10 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout11);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'grep' '-v' '^#'", "'sed' 's/^/  /'"), Some(&__sh_pbuf10)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    let __sh_bgthread = std::thread::spawn(move || {
    std::thread::sleep(std::time::Duration::from_secs_f64(("1".to_string()).trim().parse::<f64>().unwrap_or(0.0).max(0.0)));
    print!("Starting\n");
});
__SH_BGTHREADS.lock().unwrap().push(__sh_bgthread);
    let __sh_bgthread = std::thread::spawn(move || {
    std::thread::sleep(std::time::Duration::from_secs_f64(("2".to_string()).trim().parse::<f64>().unwrap_or(0.0).max(0.0)));
    print!("Processing\n");
});
__SH_BGTHREADS.lock().unwrap().push(__sh_bgthread);
    __sh_wait_all();
    print!("All done\n");
    __SH_RC.store(0, Ordering::Relaxed);
    if ({ let __sh_mapped = __sh_regex(&std::env::var("var").unwrap_or_default().to_string(), &"^[0-9]+$".to_string().to_string(), ""); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } } && std::env::var("var").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) > 0) && std::path::Path::new(&std::env::var("file").unwrap_or_default()).is_file() {
        if { let __sh_mapped = __sh_regex(&std::env::var("array[@]").unwrap_or_default().to_string(), &"value".to_string().to_string(), ""); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } } || __SHV_array.with(|v| v.borrow().len() as i64) > 5 {
            if !__sh_capture_rc(&"echo \"$var\" | grep -q \"pattern\"").0.is_empty() {
                print!("Deeply nested condition met\n");
            }
        }
    }
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
    let __sh_arith_value12 = { let __sh_value = 0; __SHV_i.with(|v| *v.borrow_mut() = __sh_value.to_string()); __sh_value };
    let __sh_num13 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value12 };
    __SH_RC.store(if __sh_num13 != 0 { 0 } else { 1 }, Ordering::Relaxed);
    let mut __sh_while_ran14 = false;
    let mut __sh_while_last15 = 0;
    while __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) < __SHV_array.with(|v| v.borrow().len() as i64) {
        __sh_while_ran14 = true;
        let __sh_arith_value16 = { let __sh_value = 0; __SHV_j.with(|v| *v.borrow_mut() = __sh_value.to_string()); __sh_value };
        let __sh_num17 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value16 };
        __SH_RC.store(if __sh_num17 != 0 { 0 } else { 1 }, Ordering::Relaxed);
        while __SHV_j.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) < std::env::var("array[i]").unwrap_or_default().chars().count() as i64 {
            if __SHV_array.with(|v| v.borrow().get(__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) > std::env::var("threshold").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) {
                { let __sh_piece = (__SHV_result.with(|v| v.borrow().get(__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) + __SHV_array.with(|v| v.borrow().get(__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0)).to_string(); __SHV_result.with(|v| { let mut b = v.borrow_mut(); let i = __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) as usize; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __sh_piece; }); }
            }
            let __sh_arith_value20 = { let __sh_out = __SHV_j.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)); __SHV_j.with(|v| *v.borrow_mut() = (__SHV_j.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) +1).to_string()); __sh_out };
            let __sh_num21 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value20 };
            __SH_RC.store(if __sh_num21 != 0 { 0 } else { 1 }, Ordering::Relaxed);
        }
        let __sh_arith_value22 = { let __sh_out = __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)); __SHV_i.with(|v| *v.borrow_mut() = (__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) +1).to_string()); __sh_out };
        let __sh_num23 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value22 };
        __SH_RC.store(if __sh_num23 != 0 { 0 } else { 1 }, Ordering::Relaxed);
        __sh_while_last15 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran14 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last15, Ordering::Relaxed); }
    __SHV___ps_tmp0.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp0.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        let mut __sh_pbuf24: Vec<u8> = Vec::new();
        __SH_PIPESTATUS.lock().unwrap().clear();
        __SH_RC.store(__sh_spawn(&format!("{} | {}", __sh_wq(&vec!["grep".to_string(), "-v".to_string(), "^#".to_string(), std::env::var("input_file").unwrap_or_default()]), __sh_wq(&vec!["head".to_string(), "-n".to_string(), std::env::var("max_lines").unwrap_or_default()])), Some(&__sh_pbuf24)), Ordering::Relaxed);
        __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV___ps_tmp0.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV___ps_tmp0.with(|v| v.borrow().clone())));
        while {
__SH_RC.store(0, Ordering::Relaxed);
{ let __sh_buf = {
let (__sh_line, __sh_gotline) = { let mut __sh_buf: Vec<u8> = Vec::new(); let mut __sh_one = [0u8; 1]; let mut __sh_any = false; let mut __sh_src: Box<dyn std::io::Read + Send> = match __SH_STDIN.lock().unwrap().take() { Some(__sh_r) => __sh_r, None => Box::new(std::io::stdin()) }; loop { match std::io::Read::read(&mut __sh_src, &mut __sh_one) { Ok(0) => break, Ok(_) => { __sh_any = true; if __sh_one[0] == b'\n' { break; } __sh_buf.push(__sh_one[0]); }, Err(_) => break } } *__SH_STDIN.lock().unwrap() = Some(__sh_src); (String::from_utf8_lossy(&__sh_buf).to_string(), __sh_any) };
let __sh_field: Vec<String> = vec![__sh_line.clone()];
__SHV_line.with(|v| *v.borrow_mut() = __sh_field.get(0).cloned().unwrap_or_default());
__SH_RC.store(if __sh_gotline { 0 } else { 1 }, Ordering::Relaxed);
__sh_gotline
} && !__SHV_line.with(|v| v.borrow().clone()).is_empty(); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if __sh_buf {
    { let __sh_value = (__SHV_counter.with(|v| v.get()) < std::env::var("max_lines").unwrap_or_default().trim().parse::<i64>().unwrap_or(0)) as i64; __SH_RC.store(if __sh_value != 0 { 0 } else { 1 }, Ordering::Relaxed); __sh_value };
}
}
__SH_RC.load(Ordering::Relaxed) == 0
} {
            if { let __sh_mapped = __sh_regex(&__SHV_line.with(|v| v.borrow().clone()).to_string(), &"^[[:space:]]*#".to_string().to_string(), ""); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } } {
                continue;
            }
            if __sh_fnmatch("*\\$\\(*\\)*", &__SHV_line.with(|v| v.borrow().clone())) {
                __sh_print_words(&[vec![format!("Contains command substitution: {}", __SHV_line.with(|v| v.borrow().clone()))]], true, false);
            } else if __sh_fnmatch("*\\$\\{[^}]*\\}*", &__SHV_line.with(|v| v.borrow().clone())) {
                __sh_print_words(&[vec![format!("Contains parameter expansion: {}", __SHV_line.with(|v| v.borrow().clone()))]], true, false);
            } else if __sh_fnmatch("*\\$\\(\\(*\\)\\)*", &__SHV_line.with(|v| v.borrow().clone())) {
                __sh_print_words(&[vec![format!("Contains arithmetic expansion: {}", __SHV_line.with(|v| v.borrow().clone()))]], true, false);
            }
            let __sh_arith_value27 = { let __sh_out = __SHV_counter.with(|v| v.get()); __SHV_counter.with(|v| v.set(__SHV_counter.with(|v| v.get()) +1)); __sh_out };
            let __sh_num28 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value27 };
            __SH_RC.store(if __sh_num28 != 0 { 0 } else { 1 }, Ordering::Relaxed);
        }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    let __ps_saved_rc = __SH_RC.load(Ordering::Relaxed);
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{}", __e) } };
        for __sh_path in &vec![__SHV___ps_tmp0.with(|v| v.borrow().clone())] {
            match std::fs::symlink_metadata(__sh_path) {
                Ok(__sh_mapped) => {
            if __sh_mapped.is_dir() {
                eprintln!("rm: cannot remove '{}': Is a directory", __sh_path);
                __sh_status = 1;
                continue;
            }
            let __sh_res = std::fs::remove_file(__sh_path);
                    if let Err(__sh_err) = __sh_res {
                        if __sh_err.kind() == std::io::ErrorKind::NotFound { /* -f: a missing path is not an error */ } else {
                            eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                            __sh_status = 1;
                        }
                    }
                }
                Err(__sh_err) => {
                        if __sh_err.kind() == std::io::ErrorKind::NotFound { /* -f: a missing path is not an error */ } else {
                            eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                            __sh_status = 1;
                        }
                }
            }
        }
        __SH_RC.store(__sh_status, Ordering::Relaxed);
    }
    __SH_RC.store(__ps_saved_rc, Ordering::Relaxed);
    { let __sh_piece = format!("{}", (std::env::var("var").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) + __SHV_array.with(|v| v.borrow().get(std::env::var("index").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0)).to_string()); __SHV_result.with(|v| { let mut b = v.borrow_mut(); let i = 0 as usize; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __sh_piece; }); }
    let mut __sh_pbuf29: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout30 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let mut __sh_pbuf31: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout32 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let mut __sh_pbuf33: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout34 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("Level 3\n");
    __sh_puts("Level 4\n");
    __sh_puts("Still level 4\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf33 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout34);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn("'grep' 'Level'", Some(&__sh_pbuf33)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    __sh_pbuf31 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout32);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn("'sed' 's/Level/Depth/'", Some(&__sh_pbuf31)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    __sh_pbuf29 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout30);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn("'wc' '-l'", Some(&__sh_pbuf29)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    __SHV___ps_tmp1.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'sort' 'file1.txt'", __sh_q(&__SHV___ps_tmp1.with(|v| v.borrow().clone()))), None), Ordering::Relaxed);
    __SHV___ps_tmp2.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'sort' 'file2.txt'", __sh_q(&__SHV___ps_tmp2.with(|v| v.borrow().clone()))), None), Ordering::Relaxed);
    __SH_RC.store(__sh_spawn(&format!("{} 2>&{}", format!("{} > {}", __sh_wq(&vec!["diff".to_string(), __SHV___ps_tmp1.with(|v| v.borrow().clone()), __SHV___ps_tmp2.with(|v| v.borrow().clone())]), __sh_q("comparison.txt")), 1), None), Ordering::Relaxed);
    let __ps_saved_rc = __SH_RC.load(Ordering::Relaxed);
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{}", __e) } };
        for __sh_path in &vec![__SHV___ps_tmp1.with(|v| v.borrow().clone()), __SHV___ps_tmp2.with(|v| v.borrow().clone())] {
            match std::fs::symlink_metadata(__sh_path) {
                Ok(__sh_mapped) => {
            if __sh_mapped.is_dir() {
                eprintln!("rm: cannot remove '{}': Is a directory", __sh_path);
                __sh_status = 1;
                continue;
            }
            let __sh_res = std::fs::remove_file(__sh_path);
                    if let Err(__sh_err) = __sh_res {
                        if __sh_err.kind() == std::io::ErrorKind::NotFound { /* -f: a missing path is not an error */ } else {
                            eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                            __sh_status = 1;
                        }
                    }
                }
                Err(__sh_err) => {
                        if __sh_err.kind() == std::io::ErrorKind::NotFound { /* -f: a missing path is not an error */ } else {
                            eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                            __sh_status = 1;
                        }
                }
            }
        }
        __SH_RC.store(__sh_status, Ordering::Relaxed);
    }
    __SH_RC.store(__ps_saved_rc, Ordering::Relaxed);
    if ((!std::env::var("var").unwrap_or_default().is_empty()) && std::path::Path::new(&std::env::var("file").unwrap_or_default()).is_file()) || std::path::Path::new(&std::env::var("dir").unwrap_or_default()).is_dir() && __sh_capture_rc(&"wc -l < \"$file\"").0.trim().parse::<i64>().unwrap_or(0) > 10 {
        print!("Complex test passed\n");
    }
    print!("a1x a1y a1z a2x a2y a2z a3x a3y a3z b1x b1y b1z b2x b2y b2z b3x b3y b3z c1x c1y c1z c2x c2y c2z c3x c3y c3z\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![{ let __sh_from_set: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); let __sh_to_set: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect(); {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![format!("UPPER: {}", std::env::var("var").unwrap_or_default().to_uppercase())]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
}.chars().map(|__sh_ch| match __sh_from_set.iter().position(|&__sh_from_ch| __sh_from_ch == __sh_ch) { Some(__sh_begin) => __sh_to_set[__sh_begin.min(__sh_to_set.len() - 1)], None => __sh_ch }).collect::<String>() }]], true, false);
    __SH_RC.store((0) as i32, Ordering::Relaxed);
    println!("{}", { let __sh_operand: Vec<String> = vec!["--long-option=value with spaces".to_string(), "--array-option".to_string(), "item1".to_string(), "item2".to_string(), "item3".to_string(), "--flag".to_string(), "positional argument".to_string(), { let __sh_value = std::env::var("var").unwrap_or_default(); if __sh_value.is_empty() { "default".to_string() } else { __sh_value.to_string() } }, "computed".to_string()]; complex_function(&__sh_operand) });
    let mut __sh_pbuf35: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout36 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __sh_sv37 = __SHV___sh2_tc0_.with(|v| v.borrow().clone());
    let __sh_sv38 = __SHV_array.with(|v| v.borrow().clone());
    let __sh_sv39 = __SHV_i.with(|v| v.borrow().clone());
    let __sh_sv40 = __SHV_result.with(|v| v.borrow().clone());
    __SHV___sh2_tc0_.with(|v| *v.borrow_mut() = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf41: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout42 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_print_words(&[vec![std::env::var("var").unwrap_or_default()]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf41 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout42);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn("'tr' '[:upper:]' '[:lower:]'", Some(&__sh_pbuf41)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
});
    if { let __sh_mapped = __sh_regex(&__SHV___sh2_tc0_.with(|v| v.borrow().clone()).to_string(), &"^[a-z]+$".to_string().to_string(), ""); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } } {
        let __sh_arith_value43 = { let __sh_value = 0; __SHV_i.with(|v| *v.borrow_mut() = __sh_value.to_string()); __sh_value };
        let __sh_num44 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value43 };
        __SH_RC.store(if __sh_num44 != 0 { 0 } else { 1 }, Ordering::Relaxed);
        let mut __sh_while_ran45 = false;
        let mut __sh_while_last46 = 0;
        while __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) < __SHV_array.with(|v| v.borrow().len() as i64) {
            __sh_while_ran45 = true;
            if __SHV_array.with(|v| v.borrow().get(__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) > std::env::var("threshold").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) && std::path::Path::new(&std::env::var("files[i]").unwrap_or_default()).is_file() {
                { let __sh_piece = (__SHV_result.with(|v| v.borrow().get(__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) + __sh_capture_rc(&"(wc -l < \"${files[i]}\")").0.trim().parse::<i64>().unwrap_or(0)).to_string(); __SHV_result.with(|v| { let mut b = v.borrow_mut(); let i = __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) as usize; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __sh_piece; }); }
                __SH_RC.store(0, Ordering::Relaxed);
            } else {
                __SH_RC.store(0, Ordering::Relaxed);
            }
            let __sh_arith_value47 = { let __sh_out = __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)); __SHV_i.with(|v| *v.borrow_mut() = (__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) +1).to_string()); __sh_out };
            let __sh_num48 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value47 };
            __SH_RC.store(if __sh_num48 != 0 { 0 } else { 1 }, Ordering::Relaxed);
            __sh_while_last46 = __SH_RC.load(Ordering::Relaxed);
        }
        if !__sh_while_ran45 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last46, Ordering::Relaxed); }
    } else {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    __SHV___sh2_tc0_.with(|v| *v.borrow_mut() = __sh_sv37);
    __SHV_array.with(|v| *v.borrow_mut() = __sh_sv38);
    __SHV_i.with(|v| *v.borrow_mut() = __sh_sv39);
    __SHV_result.with(|v| *v.borrow_mut() = __sh_sv40);
    __sh_pbuf35 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout36);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'sort' '-n'", format!("{} > {}", "'tail' '-n' '5'".to_string(), __sh_q("final_result.txt"))), Some(&__sh_pbuf35)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn complex_function(__argv: &[String]) -> String {
    __SHV_args.with(|v| *v.borrow_mut() = __argv.to_vec());
    __SH_RC.store(0, Ordering::Relaxed);
    { let __words = Vec::<String>::new(); __SHV_options.with(|v| { let mut __sh_mapped = v.borrow_mut(); __sh_mapped.clear(); for __sh_word in __words { if let Some(rest) = __sh_word.strip_prefix('[') { if let Some(eq) = rest.find(']') { let __sh_key = rest[..eq].to_string(); let __sh_value = rest[eq + 2..].to_string(); __sh_mapped.insert(__sh_key, __sh_value); } } } }); }
    __SHV_i.with(|v| *v.borrow_mut() = "0".to_string());
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_while_ran49 = false;
    let mut __sh_while_last50 = 0;
    while __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) < __SHV_args.with(|v| v.borrow().len() as i64) {
        __sh_while_ran49 = true;
        __SHV___sh2_casesubj_2_.with(|v| *v.borrow_mut() = format!("{}", { let __sh_idx = __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)); let __sh_num = __SHV_args.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_args.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } }));
        __SH_RC.store(0, Ordering::Relaxed);
        if __SHV___sh2_casesubj_2_.with(|v| v.borrow().clone()).starts_with("--") {
            __SHV_key.with(|v| *v.borrow_mut() = { let __sh_str: String = { let __sh_idx = __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)); let __sh_num = __SHV_args.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_args.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } }.to_string(); if let Some(__sh_res) = __sh_str.strip_prefix("--") { __sh_res.to_string() } else { __sh_str } });
            __SH_RC.store(0, Ordering::Relaxed);
            __SHV_value.with(|v| *v.borrow_mut() = { let __sh_idx = __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) + 1; let __sh_num = __SHV_args.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_args.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } });
            __SH_RC.store(0, Ordering::Relaxed);
            __SHV_options.with(|v| { v.borrow_mut().insert(format!("{}", __SHV_key.with(|v| v.borrow().clone())), __SHV_value.with(|v| v.borrow().clone())); });
            __SH_RC.store(0, Ordering::Relaxed);
            let __sh_arith_value51 = { __SHV_i.with(|v| *v.borrow_mut() = (__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) + 2).to_string()); __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) };
            let __sh_num52 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value51 };
            __SH_RC.store(if __sh_num52 != 0 { 0 } else { 1 }, Ordering::Relaxed);
        } else if __SHV___sh2_casesubj_2_.with(|v| v.borrow().clone()).starts_with("-") {
            __SHV_flags.with(|v| *v.borrow_mut() = { let __sh_str: String = { let __sh_idx = __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)); let __sh_num = __SHV_args.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_args.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } }.to_string(); if let Some(__sh_res) = __sh_str.strip_prefix("-") { __sh_res.to_string() } else { __sh_str } });
            __SHV_j.with(|v| *v.borrow_mut() = "0".to_string());
            __SH_RC.store(0, Ordering::Relaxed);
            let mut __sh_while_ran53 = false;
            let mut __sh_while_last54 = 0;
            while __SHV_j.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) < __SHV_flags.with(|v| v.borrow().clone()).chars().count() as i64 {
                __sh_while_ran53 = true;
                __SHV_options.with(|v| { v.borrow_mut().insert(format!("{}", { let __sh_s = &__SHV_flags.with(|v| v.borrow().clone()); let __sh_o = __SHV_j.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)); let __sh_l = 1; let __sh_ch: Vec<char> = __sh_s.chars().collect(); let __sh_n = __sh_ch.len() as i64; let mut __sh_st = if __sh_o < 0 { __sh_n + __sh_o } else { __sh_o }; if __sh_st < 0 { __sh_st = 0; } if __sh_st > __sh_n { String::new() } else { let mut __sh_cn = if __sh_l == i64::MIN { __sh_n - __sh_st } else if __sh_l < 0 { __sh_n + __sh_l - __sh_st } else { __sh_l }; if __sh_cn < 0 { __sh_cn = 0; } if __sh_st + __sh_cn > __sh_n { __sh_cn = __sh_n - __sh_st; } __sh_ch[__sh_st as usize..(__sh_st + __sh_cn) as usize].iter().collect::<String>() } }), "true".to_string()); });
                __SH_RC.store(0, Ordering::Relaxed);
                let __sh_arith_value55 = { let __sh_out = __SHV_j.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)); __SHV_j.with(|v| *v.borrow_mut() = (__SHV_j.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) +1).to_string()); __sh_out };
                let __sh_num56 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value55 };
                __SH_RC.store(if __sh_num56 != 0 { 0 } else { 1 }, Ordering::Relaxed);
                __sh_while_last54 = __SH_RC.load(Ordering::Relaxed);
            }
            if !__sh_while_ran53 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last54, Ordering::Relaxed); }
            let __sh_arith_value57 = { let __sh_out = __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)); __SHV_i.with(|v| *v.borrow_mut() = (__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) +1).to_string()); __sh_out };
            let __sh_num58 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value57 };
            __SH_RC.store(if __sh_num58 != 0 { 0 } else { 1 }, Ordering::Relaxed);
        } else {
            __SH_RC.store(0, Ordering::Relaxed);
            __sh_while_last50 = __SH_RC.load(Ordering::Relaxed);
            break;
        }
        __sh_while_last50 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran49 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last50, Ordering::Relaxed); }
    let __sh_value = format!("Processed {} options", __SHV_options.with(|v| v.borrow().len() as i64).to_string()); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed); return __sh_value;
}