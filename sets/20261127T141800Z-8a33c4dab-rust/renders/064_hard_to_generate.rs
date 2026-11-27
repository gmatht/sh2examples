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
thread_local! { #[allow(non_upper_case_globals)] static __SHV_file: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_first_half: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_gid: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_home: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_info: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_last_half: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_lowercase: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_middle: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_numbers: std::cell::RefCell<Vec<i64>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_pass: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_result: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_shell: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_uid: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_user: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn __sh_wq(ws: &[String]) -> String {
    ws.iter().map(|w| __sh_q(w)).collect::<Vec<_>>().join(" ")
}

fn __sh_cat(ws: &[Vec<String>]) -> Vec<String> {
    ws.concat()
}

fn __sh_print_words(ws: &[Vec<String>], nl: bool, _esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    if nl { s.push('\n'); }
    __sh_sink(s.as_bytes());
}

fn __sh_cap_bytes(cmd: &str, input: Option<&[u8]>) -> Vec<u8> {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    if input.is_some() { c.stdin(std::process::Stdio::piped()); }
    let Ok(mut ch) = c.spawn() else { return Vec::new() };
    if let Some(data) = input {
        if let Some(mut si) = ch.stdin.take() { let _ = std::io::Write::write_all(&mut si, data); }
    }
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let _ = ch.wait();
    out
}

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    let Ok(mut ch) = c.spawn() else { return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let s = String::from_utf8_lossy(&out).trim_end_matches('\n').to_string();
    (s, rc)
}

fn __sh_spawn(cmd: &str, input: Option<&[u8]>) -> i32 {
    use std::process::Stdio;
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
    c.stdout(if want_out { Stdio::piped() } else { Stdio::inherit() });
    if let (None, Some(p)) = (&input, &stdin_path) {
        if let Ok(f) = std::fs::File::open(p) {
            c.stdin(Stdio::from(f));
        }
    }
    if input.is_some() { c.stdin(Stdio::piped()); }
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

fn __sh_run(cmd: &str) -> i32 {
    __sh_spawn(cmd, None)
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

fn __sh_replace(s: &str, pat0: &str, repl: &str, all: bool) -> String {
    let anchored_s = pat0.starts_with('#');
    let anchored_e = pat0.starts_with('%');
    let pat = pat0.trim_start_matches('#').trim_start_matches('%');
    let mut b: Vec<usize> = s.char_indices().map(|(k, _)| k).collect();
    b.push(s.len());
    let n = b.len() - 1;
    let mut out = String::new();
    let mut i = 0;
    let mut done = false;
    while i <= n {
        if anchored_s && i > 0 { break; }
        let hit = (i..=n).find(|&e| __sh_fnmatch(pat, &s[b[i]..b[e]]))
            .filter(|&e| !anchored_e || e == n);
        match hit {
            Some(e) => {
                out.push_str(repl);
                i = e;
                done = true;
                if !all {
                    out.push_str(&s[b[i]..]);
                    break;
                }
                if i >= n { break; }
            }
            None => {
                if i < n { out.push_str(&s[b[i]..b[i + 1]]); }
                i += 1;
                if anchored_s { break; }
            }
        }
    }
    if done { out } else { s.to_string() }
}

fn __sh_arg(i: usize) -> String {
    __SH_ARGV.lock().unwrap().get(i).cloned().unwrap_or_default()
}

fn __sh_glob(pat: &str) -> Vec<String> {
    let cmd = format!("printf '%s\\n' {pat}");
    let out = __sh_cap_bytes(&cmd, None);
    let s = String::from_utf8_lossy(&out);
    let v: Vec<String> = s.lines().map(|x| x.to_string()).collect();
    if v.is_empty() { vec![pat.to_string()] } else { v }
}

fn __sh_hostname() -> String {
    if let Ok(h) = std::env::var("HOSTNAME") { return h; }
    std::fs::read_to_string("/proc/sys/kernel/hostname")
        .map(|s| s.trim_end().to_string()).unwrap_or_default()
}

fn __sh_regex(text: &str, pat: &str, flags: &str) -> Option<bool> {
    let pat = if flags.contains('i') { format!("(?i){pat}") } else { pat.to_string() };
    match regex::Regex::new(&pat) {
        Ok(re) => Some(re.is_match(text)),
        Err(_) => None,
    }
}

fn __sh_run_traps() {
    let hs = std::mem::take(&mut *__SH_TRAPS.lock().unwrap());
    for h in hs { let _ = __sh_spawn(&h, None); }
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
    __SHV___ps_tmp1.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp1.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        __SHV___ps_tmp0.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
        {
            let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
            __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp0.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
            let __sh_pbuf0: Vec<u8> = Vec::new();
            __SH_PIPESTATUS.lock().unwrap().clear();
            __SH_RC.store(__sh_spawn(&format!("{} | {}", "'grep' '-v' '^#' '/etc/passwd'", "'cut' '-d' ':' '-f' '1'"), Some(&__sh_pbuf0)), Ordering::Relaxed);
            __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
            __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
        }
        __SH_RC.store(__sh_run(&__sh_wq(&vec!["sort".to_string(), __SHV___ps_tmp0.with(|v| v.borrow().clone())])), Ordering::Relaxed);
        let __ps_saved_rc = __SH_RC.load(Ordering::Relaxed);
        {
            let mut __sh_status: i32 = 0;
            let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
                std::io::ErrorKind::NotFound => "No such file or directory".into(),
                std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
                std::io::ErrorKind::AlreadyExists => "File exists".into(),
                std::io::ErrorKind::IsADirectory => "Is a directory".into(),
                std::io::ErrorKind::NotADirectory => "Not a directory".into(),
                _ => format!("{__e}") } };
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
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    __SHV___ps_tmp3.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp3.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        __SHV___ps_tmp2.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
        {
            let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
            __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp2.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
            let __sh_pbuf1: Vec<u8> = Vec::new();
            __SH_PIPESTATUS.lock().unwrap().clear();
            __SH_RC.store(__sh_spawn(&format!("{} | {}", "'grep' '-v' '^#' '/etc/group'", "'cut' '-d' ':' '-f' '1'"), Some(&__sh_pbuf1)), Ordering::Relaxed);
            __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
            __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
        }
        __SH_RC.store(__sh_run(&__sh_wq(&vec!["sort".to_string(), __SHV___ps_tmp2.with(|v| v.borrow().clone())])), Ordering::Relaxed);
        let __ps_saved_rc = __SH_RC.load(Ordering::Relaxed);
        {
            let mut __sh_status: i32 = 0;
            let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
                std::io::ErrorKind::NotFound => "No such file or directory".into(),
                std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
                std::io::ErrorKind::AlreadyExists => "File exists".into(),
                std::io::ErrorKind::IsADirectory => "Is a directory".into(),
                std::io::ErrorKind::NotADirectory => "Not a directory".into(),
                _ => format!("{__e}") } };
            for __sh_path in &vec![__SHV___ps_tmp2.with(|v| v.borrow().clone())] {
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
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    __SH_RC.store(__sh_run(&__sh_wq(&vec!["diff".to_string(), __SHV___ps_tmp1.with(|v| v.borrow().clone()), __SHV___ps_tmp3.with(|v| v.borrow().clone())])), Ordering::Relaxed);
    let __ps_saved_rc = __SH_RC.load(Ordering::Relaxed);
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{__e}") } };
        for __sh_path in &vec![__SHV___ps_tmp1.with(|v| v.borrow().clone()), __SHV___ps_tmp3.with(|v| v.borrow().clone())] {
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
    print!("Files:  file_a_1..10.txt file_a_1..10.log file_a_1..10.dat file_a_20.txt file_a_20.log file_a_20.dat file_a_30..40.txt file_a_30..40.log file_a_30..40.dat file_b_1..10.txt file_b_1..10.log file_b_1..10.dat file_b_20.txt file_b_20.log file_b_20.dat file_b_30..40.txt file_b_30..40.log file_b_30..40.dat file_c_1..10.txt file_c_1..10.log file_c_1..10.dat file_c_20.txt file_c_20.log file_c_20.dat file_c_30..40.txt file_c_30..40.log file_c_30..40.dat file_d_1..10.txt file_d_1..10.log file_d_1..10.dat file_d_20.txt file_d_20.log file_d_20.dat file_d_30..40.txt file_d_30..40.log file_d_30..40.dat file_e_1..10.txt file_e_1..10.log file_e_1..10.dat file_e_20.txt file_e_20.log file_e_20.dat file_e_30..40.txt file_e_30..40.log file_e_30..40.dat file_f_1..10.txt file_f_1..10.log file_f_1..10.dat file_f_20.txt file_f_20.log file_f_20.dat file_f_30..40.txt file_f_30..40.log file_f_30..40.dat file_g_1..10.txt file_g_1..10.log file_g_1..10.dat file_g_20.txt file_g_20.log file_g_20.dat file_g_30..40.txt file_g_30..40.log file_g_30..40.dat file_h_1..10.txt file_h_1..10.log file_h_1..10.dat file_h_20.txt file_h_20.log file_h_20.dat file_h_30..40.txt file_h_30..40.log file_h_30..40.dat file_i_1..10.txt file_i_1..10.log file_i_1..10.dat file_i_20.txt file_i_20.log file_i_20.dat file_i_30..40.txt file_i_30..40.log file_i_30..40.dat file_j_1..10.txt file_j_1..10.log file_j_1..10.dat file_j_20.txt file_j_20.log file_j_20.dat file_j_30..40.txt file_j_30..40.log file_j_30..40.dat file_k_1..10.txt file_k_1..10.log file_k_1..10.dat file_k_20.txt file_k_20.log file_k_20.dat file_k_30..40.txt file_k_30..40.log file_k_30..40.dat file_l_1..10.txt file_l_1..10.log file_l_1..10.dat file_l_20.txt file_l_20.log file_l_20.dat file_l_30..40.txt file_l_30..40.log file_l_30..40.dat file_m_1..10.txt file_m_1..10.log file_m_1..10.dat file_m_20.txt file_m_20.log file_m_20.dat file_m_30..40.txt file_m_30..40.log file_m_30..40.dat file_n_1..10.txt file_n_1..10.log file_n_1..10.dat file_n_20.txt file_n_20.log file_n_20.dat file_n_30..40.txt file_n_30..40.log file_n_30..40.dat file_o_1..10.txt file_o_1..10.log file_o_1..10.dat file_o_20.txt file_o_20.log file_o_20.dat file_o_30..40.txt file_o_30..40.log file_o_30..40.dat file_p_1..10.txt file_p_1..10.log file_p_1..10.dat file_p_20.txt file_p_20.log file_p_20.dat file_p_30..40.txt file_p_30..40.log file_p_30..40.dat file_q_1..10.txt file_q_1..10.log file_q_1..10.dat file_q_20.txt file_q_20.log file_q_20.dat file_q_30..40.txt file_q_30..40.log file_q_30..40.dat file_r_1..10.txt file_r_1..10.log file_r_1..10.dat file_r_20.txt file_r_20.log file_r_20.dat file_r_30..40.txt file_r_30..40.log file_r_30..40.dat file_s_1..10.txt file_s_1..10.log file_s_1..10.dat file_s_20.txt file_s_20.log file_s_20.dat file_s_30..40.txt file_s_30..40.log file_s_30..40.dat file_t_1..10.txt file_t_1..10.log file_t_1..10.dat file_t_20.txt file_t_20.log file_t_20.dat file_t_30..40.txt file_t_30..40.log file_t_30..40.dat file_u_1..10.txt file_u_1..10.log file_u_1..10.dat file_u_20.txt file_u_20.log file_u_20.dat file_u_30..40.txt file_u_30..40.log file_u_30..40.dat file_v_1..10.txt file_v_1..10.log file_v_1..10.dat file_v_20.txt file_v_20.log file_v_20.dat file_v_30..40.txt file_v_30..40.log file_v_30..40.dat file_w_1..10.txt file_w_1..10.log file_w_1..10.dat file_w_20.txt file_w_20.log file_w_20.dat file_w_30..40.txt file_w_30..40.log file_w_30..40.dat file_x_1..10.txt file_x_1..10.log file_x_1..10.dat file_x_20.txt file_x_20.log file_x_20.dat file_x_30..40.txt file_x_30..40.log file_x_30..40.dat file_y_1..10.txt file_y_1..10.log file_y_1..10.dat file_y_20.txt file_y_20.log file_y_20.dat file_y_30..40.txt file_y_30..40.log file_y_30..40.dat file_z_1..10.txt file_z_1..10.log file_z_1..10.dat file_z_20.txt file_z_20.log file_z_20.dat file_z_30..40.txt file_z_30..40.log file_z_30..40.dat\n");
    let name = "John Doe";
    __SH_RC.store(0, Ordering::Relaxed);
    println!("Hello {}", "John_Doe");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("Length: {}", (name.len() as i64));
    __SH_RC.store(0, Ordering::Relaxed);
    println!("First: {}", name.chars().skip((0) as usize).take((4) as usize).collect::<String>());
    println!("Last: {}", { let __sh_s = &name; let __sh_o = -3; let __sh_l = i64::MIN; let __sh_n = __sh_s.chars().count() as i64; let mut __sh_st = if __sh_o < 0 { __sh_n + __sh_o } else { __sh_o }; if __sh_st < 0 { __sh_st = 0; } if __sh_st > __sh_n { String::new() } else { let mut __sh_cn = if __sh_l == i64::MIN { __sh_n - __sh_st } else if __sh_l < 0 { __sh_n + __sh_l - __sh_st } else { __sh_l }; if __sh_cn < 0 { __sh_cn = 0; } if __sh_st + __sh_cn > __sh_n { __sh_cn = __sh_n - __sh_st; } __sh_s.chars().skip(__sh_st as usize).take(__sh_cn as usize).collect::<String>() } });
    __SH_RC.store(0, Ordering::Relaxed);
    let __sh_case2 = __sh_arg(0);
    if __sh_fnmatch("[a-z]*", &__sh_case2) {
        print!("Lowercase start\n");
    } else if __sh_fnmatch("[A-Z]*", &__sh_case2) {
        print!("Uppercase start\n");
    } else if __sh_fnmatch("[0-9]*", &__sh_case2) {
        print!("Number start\n");
    } else if __sh_fnmatch("?", &__sh_case2) {
        print!("Single character\n");
    } else if true {
        print!("Something else\n");
    } else {
    }
    let mut i = { let __sh_value = (2).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } };
    __SH_RC.store(0, Ordering::Relaxed);
    let __sh_arith_value3 = ({ let __sh_out = i.trim().parse::<i64>().unwrap_or(0); i = (i.trim().parse::<i64>().unwrap_or(0) +1).to_string(); __sh_out } + { i = (i.trim().parse::<i64>().unwrap_or(0) +1).to_string(); i.trim().parse::<i64>().unwrap_or(0) }).to_string();
    let j = __sh_arith_value3.clone();
    { let __sh_value = __sh_arith_value3; let __sh_num = __sh_value.trim().parse::<i64>().unwrap_or(0); __SH_RC.store(if __sh_num != 0 { 0 } else { 1 }, Ordering::Relaxed); __sh_value };
    println!("i={}, j={}", i.clone(), j.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_config.with(|v| { v.borrow_mut().insert("user".to_string(), "admin".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_config.with(|v| { v.borrow_mut().insert("host".to_string(), "localhost".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_config.with(|v| { v.borrow_mut().insert("port".to_string(), "8080".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'cat'", __sh_q("config.txt")), Some("User: $USER\nHost: ${HOSTNAME:-localhost}\nPath: $PWD\n".to_string().as_bytes())), Ordering::Relaxed);
    __SHV___ps_tmp4.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp4.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        let __sh_pbuf4: Vec<u8> = Vec::new();
        __SH_PIPESTATUS.lock().unwrap().clear();
        __SH_RC.store(__sh_spawn(&format!("{} | {}", "'cut' '-d' ':' '-f' '1' '/etc/passwd'", "'sort'"), Some(&__sh_pbuf4)), Ordering::Relaxed);
        __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    __SHV___ps_tmp5.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp5.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        let __sh_pbuf5: Vec<u8> = Vec::new();
        __SH_PIPESTATUS.lock().unwrap().clear();
        __SH_RC.store(__sh_spawn(&format!("{} | {}", "'cut' '-d' ':' '-f' '3' '/etc/passwd'", "'sort' '-n'"), Some(&__sh_pbuf5)), Ordering::Relaxed);
        __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    let __sh_pbuf6: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", __sh_wq(&vec!["paste".to_string(), __SHV___ps_tmp4.with(|v| v.borrow().clone()), __SHV___ps_tmp5.with(|v| v.borrow().clone())]), "'head' '-10'"), Some(&__sh_pbuf6)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    let __ps_saved_rc = __SH_RC.load(Ordering::Relaxed);
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{__e}") } };
        for __sh_path in &vec![__SHV___ps_tmp4.with(|v| v.borrow().clone()), __SHV___ps_tmp5.with(|v| v.borrow().clone())] {
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
    if { let __sh_mapped = __sh_regex(&__sh_arg(0).to_string(), &"^[0-9]+$".to_string().to_string(), "i"); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } } && (__sh_arg(1).to_lowercase() == "test".to_string().to_lowercase()) || (__sh_arg(1).to_lowercase() == "debug".to_string().to_lowercase()) {
        print!("Valid input\n");
    }
    {
        let mut __sh_status: i32 = 0;
        for __sh_path in &vec!["project/src/main/java".to_string(), "project/src/main/resources".to_string(), "project/src/test/java".to_string(), "project/src/test/resources".to_string(), "project/docs/api".to_string(), "project/docs/user".to_string(), "project/build/classes".to_string(), "project/build/lib".to_string()] {
            let __sh_res = if true { std::fs::create_dir_all(__sh_path) } else { std::fs::create_dir(__sh_path) };
            match __sh_res {
                Ok(_) => {
                    if false { println!("mkdir: created directory '{}'", __sh_path); }
                }
                Err(__sh_err) => {
                    // MEASURED shapes, which are NOT one format: this host's
                    // mkdir names the path on `File exists` and omits it on
                    // `Permission denied` / `No such file or directory`.
                    match __sh_err.kind() {
                        std::io::ErrorKind::AlreadyExists => eprintln!("mkdir: {}: File exists", __sh_path),
                        std::io::ErrorKind::PermissionDenied => eprintln!("mkdir: Permission denied"),
                        std::io::ErrorKind::NotFound => eprintln!("mkdir: No such file or directory"),
                        _ => eprintln!("mkdir: {}: {}", __sh_path, __sh_err),
                    }
                    __sh_status = 1;
                }
            }
        }
        __SH_RC.store(__sh_status, Ordering::Relaxed);
    }
    let filename = "my_file.txt".to_string();
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_basename.with(|v| *v.borrow_mut() = { let __sh_str: String = filename.clone().to_string(); let __sh_delim: &str = "."; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_extension.with(|v| *v.borrow_mut() = { let __sh_str: String = filename.clone().to_string(); let __sh_delim: &str = "."; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } });
    let mut uppercase = filename.clone().to_uppercase();
    __SHV_lowercase.with(|v| *v.borrow_mut() = filename.clone().to_lowercase());
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_result.with(|v| *v.borrow_mut() = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![((__sh_capture_rc(&"(wc -l < /etc/passwd)").0.trim().parse::<i64>().unwrap_or(0) + __sh_capture_rc(&"(wc -l < /etc/group)").0.trim().parse::<i64>().unwrap_or(0))).to_string()]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
});
    let __sh_pbuf7: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", format!("{} | {}", format!("{} | {}", "'grep' '-v' '^#' '/etc/passwd'", "'cut' '-d' ':' '-f' '1,3'"), "'sort' '-t' ':' '-k' '2' '-n'"), format!("{} 2> {}", format!("{} > {}", "'head' '-5'", __sh_q("users.txt")), __sh_q("errors.log"))), Some(&__sh_pbuf7)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/etc/passwd").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/etc/passwd"));
        while {
let (__sh_line, __sh_gotline) = { let mut __sh_buf: Vec<u8> = Vec::new(); let mut __sh_one = [0u8; 1]; let mut __sh_any = false; let mut __sh_src: Box<dyn std::io::Read + Send> = match __SH_STDIN.lock().unwrap().take() { Some(__sh_r) => __sh_r, None => Box::new(std::io::stdin()) }; loop { match std::io::Read::read(&mut __sh_src, &mut __sh_one) { Ok(0) => break, Ok(_) => { __sh_any = true; if __sh_one[0] == b'\n' { break; } __sh_buf.push(__sh_one[0]); }, Err(_) => break } } *__SH_STDIN.lock().unwrap() = Some(__sh_src); (String::from_utf8_lossy(&__sh_buf).to_string(), __sh_any) };
let __sh_field: Vec<String> = { let __sh_ifs = ":"; let __sh_n = 7usize; let __sh_l = &__sh_line; if __sh_n <= 1 || __sh_ifs.is_empty() { vec![__sh_l.to_string()] } else { let __sh_ws = __sh_ifs.chars().all(|c| c == ' ' || c == '\t' || c == '\n'); let __sh_ch: Vec<char> = __sh_l.chars().collect(); let mut __sh_f: Vec<String> = Vec::new(); let mut __sh_i = 0usize; while __sh_i < __sh_ch.len() && __sh_ifs.contains(__sh_ch[__sh_i]) { __sh_i += 1; } while __sh_f.len() < __sh_n - 1 && __sh_i < __sh_ch.len() { let __sh_s = __sh_i; while __sh_i < __sh_ch.len() && !__sh_ifs.contains(__sh_ch[__sh_i]) { __sh_i += 1; } __sh_f.push(__sh_ch[__sh_s..__sh_i].iter().collect()); if __sh_i >= __sh_ch.len() { break; } __sh_i += 1; if __sh_ws { while __sh_i < __sh_ch.len() && __sh_ifs.contains(__sh_ch[__sh_i]) { __sh_i += 1; } } } __sh_f.push(__sh_ch[__sh_i.min(__sh_ch.len())..].iter().collect()); __sh_f } };
__SHV_user.with(|v| *v.borrow_mut() = __sh_field.get(0).cloned().unwrap_or_default());
__SHV_pass.with(|v| *v.borrow_mut() = __sh_field.get(1).cloned().unwrap_or_default());
__SHV_uid.with(|v| *v.borrow_mut() = __sh_field.get(2).cloned().unwrap_or_default());
__SHV_gid.with(|v| *v.borrow_mut() = __sh_field.get(3).cloned().unwrap_or_default());
__SHV_info.with(|v| *v.borrow_mut() = __sh_field.get(4).cloned().unwrap_or_default());
__SHV_home.with(|v| *v.borrow_mut() = __sh_field.get(5).cloned().unwrap_or_default());
__SHV_shell.with(|v| *v.borrow_mut() = __sh_field.get(6).cloned().unwrap_or_default());
__SH_RC.store(if __sh_gotline { 0 } else { 1 }, Ordering::Relaxed);
__sh_gotline
} {
            if (__SHV_uid.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) > 1000) && __SHV_shell.with(|v| v.borrow().clone()).to_lowercase() != "/bin/false".to_string().to_lowercase() {
                if { let __sh_mapped = __sh_regex(&__SHV_home.with(|v| v.borrow().clone()).to_string(), &"^/home/".to_string().to_string(), "i"); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } } {
                    println!("User: {} (UID: {}) - {}", __SHV_user.with(|v| v.borrow().clone()), __SHV_uid.with(|v| v.borrow().clone()), __SHV_home.with(|v| v.borrow().clone()));
                }
            }
        }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    __SHV_numbers.with(|v| *v.borrow_mut() = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_middle.with(|v| *v.borrow_mut() = vec![{ let __sh_value = __SHV_numbers.with(|v| v.borrow().iter().map(|n| n.to_string()).collect::<Vec<String>>()); let __sh_offset = if 3 < 0 { (__sh_value.len() as i64 + 3).max(0) } else { 3 } as usize; let __sh_offset = __sh_offset.min(__sh_value.len()); let __sh_len = if 4 < 0 { __sh_value.len() as i64 - __sh_offset as i64 } else { 4 };                          let __sh_start = __sh_offset.max(0) as usize; let __sh_stop = ((__sh_start as i64 + __sh_len).max(__sh_start as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_start);                          __sh_value[__sh_start..__sh_stop].join(" ") }]);
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_first_half.with(|v| *v.borrow_mut() = vec![{ let __sh_value = __SHV_numbers.with(|v| v.borrow().iter().map(|n| n.to_string()).collect::<Vec<String>>()); let __sh_offset = if 0 < 0 { (__sh_value.len() as i64 + 0).max(0) } else { 0 } as usize; let __sh_offset = __sh_offset.min(__sh_value.len()); let __sh_len = if 5 < 0 { __sh_value.len() as i64 - __sh_offset as i64 } else { 5 };                          let __sh_start = __sh_offset.max(0) as usize; let __sh_stop = ((__sh_start as i64 + __sh_len).max(__sh_start as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_start);                          __sh_value[__sh_start..__sh_stop].join(" ") }]);
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_last_half.with(|v| *v.borrow_mut() = vec![{ let __sh_value = __SHV_numbers.with(|v| v.borrow().iter().map(|n| n.to_string()).collect::<Vec<String>>()); let __sh_offset = if 5 < 0 { (__sh_value.len() as i64 + 5).max(0) } else { 5 } as usize; let __sh_offset = __sh_offset.min(__sh_value.len()); let __sh_len = if i64::MIN < 0 { __sh_value.len() as i64 - __sh_offset as i64 } else { i64::MIN };                          let __sh_start = __sh_offset.max(0) as usize; let __sh_stop = ((__sh_start as i64 + __sh_len).max(__sh_start as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_start);                          __sh_value[__sh_start..__sh_stop].join(" ") }]);
    __SH_RC.store(0, Ordering::Relaxed);
    let __sh_items10 = __sh_cat(&[__sh_glob("*.txt"), __sh_glob("*.log"), __sh_glob("*.dat")]);
    let mut __sh_index11: usize = 0;
    loop {
    if !(__sh_index11 < __sh_items10.len()) { break; }
        __SHV_file.with(|v| *v.borrow_mut() = __sh_items10[__sh_index11].clone());
        let __sh_case12 = __SHV_file.with(|v| v.borrow().clone());
        if __sh_fnmatch("*.txt", &__sh_case12) || __sh_fnmatch("*.log", &__sh_case12) {
            println!("Text file: {}", __SHV_file.with(|v| v.borrow().clone()));
        } else if __sh_fnmatch("*.dat", &__sh_case12) {
            println!("Data file: {}", __SHV_file.with(|v| v.borrow().clone()));
        } else if true {
            println!("Other file: {}", __SHV_file.with(|v| v.borrow().clone()));
        } else {
        }
    __sh_index11 += 1;
    }
    __SHV_DEBUG.with(|v| *v.borrow_mut() = "1".to_string());
    std::env::set_var("DEBUG", "1");
    __SHV_LOG_LEVEL.with(|v| *v.borrow_mut() = "verbose".to_string());
    std::env::set_var("LOG_LEVEL", "verbose");
    let __sh_sv13 = __SHV_DEBUG.with(|v| v.borrow().clone());
    __SHV_DEBUG.with(|v| *v.borrow_mut() = String::new());
    println!("Inner: LOG_LEVEL={}, DEBUG={}", __SHV_LOG_LEVEL.with(|v| v.borrow().clone()), { let __sh_value = __SHV_DEBUG.with(|v| v.borrow().clone()); if __sh_value.is_empty() { "unset".to_string() } else { __sh_value.to_string() } });
    __SHV_DEBUG.with(|v| *v.borrow_mut() = __sh_sv13);
    println!("Outer: LOG_LEVEL={}, DEBUG={}", __SHV_LOG_LEVEL.with(|v| v.borrow().clone()), __SHV_DEBUG.with(|v| v.borrow().clone()));
    let message = format!("Hello {} from {}", { let __sh_value = std::env::var("USER").unwrap_or_default(); if __sh_value.is_empty() { "guest".to_string() } else { __sh_value.to_string() } }, { let __sh_value = __sh_hostname(); if __sh_value.is_empty() { "localhost".to_string() } else { __sh_value.to_string() } });
    println!("{}", message.clone());
    __SH_TRAPS.lock().unwrap().push("echo \"Cleaning up...\"; rm -f /tmp/064_hard_temp_*".to_string());
    __SH_RC.store(0, Ordering::Relaxed);
    let input = { let __sh_value = __sh_arg(0); if __sh_value.is_empty() { "default_value".to_string() } else { __sh_value.to_string() } };
    let sanitized = __sh_replace(&input.clone(), "[^a-zA-Z0-9]", "_", true);
    uppercase = sanitized.clone().to_uppercase();
    __SH_RC.store(0, Ordering::Relaxed);
    println!("Input: '{}' -> Sanitized: '{}' -> Uppercase: '{}'", input.clone(), sanitized.clone(), uppercase.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_buf = std::path::Path::new(&__sh_arg(0)).is_file() && {
print!("File exists\n");
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("File not found\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    { let __sh_buf = (std::path::Path::new(&__sh_arg(1)).is_dir() && { let __sh_dir = __sh_arg(1); let __sh_res = std::env::set_current_dir(&__sh_dir); if __sh_res.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __sh_res.is_ok() { 0 } else { 1 }, Ordering::Relaxed); __sh_res.is_ok() }) && {
println!("{}", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default());
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("Directory not accessible\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    print!("Script completed successfully!\n");
    __SH_RC.store(0, Ordering::Relaxed);
    let __sh_final = __SH_RC.load(Ordering::Relaxed);
    __sh_run_traps();
    std::process::ExitCode::from(__sh_final as u8)
}