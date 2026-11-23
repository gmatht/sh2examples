use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_arg: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_cmd: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_stdout: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn __sh_wq(ws: &[String]) -> String {
    ws.iter().map(|w| __sh_q(w)).collect::<Vec<_>>().join(" ")
}

fn __sh_atoi(s: &str) -> i64 {
    let t = s.trim();
    if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        return i64::from_str_radix(h, 16).unwrap_or(0);
    }
    t.parse::<i64>().unwrap_or(0)
}

fn __sh_print_words(ws: &[Vec<String>], nl: bool, _esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    if nl { s.push('\n'); }
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

fn __sh_regex(text: &str, pat: &str, flags: &str) -> Option<bool> {
    let pat = if flags.contains('i') { format!("(?i){}", pat) } else { pat.to_string() };
    match regex::Regex::new(&pat) {
        Ok(re) => Some(re.is_match(text)),
        Err(_) => None,
    }
}

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

fn main() -> std::process::ExitCode {
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_operand: Vec<String> = vec!["realpath (default, --physical) on a simple file".to_string(), "realpath".to_string(), "/bin".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["realpath on a two‑hop symlink chain: /usr/bin/vi → /etc/alternatives/vi → /usr/bin/vim.basic".to_string(), "realpath".to_string(), "/usr/bin/vi".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["realpath on a relative symlink: /usr/local/bin/pi".to_string(), "realpath".to_string(), "/usr/local/bin/pi".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["realpath on a regular file (no symlinks)".to_string(), "realpath".to_string(), "/etc/hostname".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["realpath on a directory with .. component".to_string(), "realpath".to_string(), "/tmp/..".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--canonicalize-existing on an existing path".to_string(), "realpath".to_string(), "--canonicalize-existing".to_string(), "/usr/bin/sh".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_buf = { let __sh_operand: Vec<String> = vec!["--canonicalize-existing on a path with a missing last component (should fail)".to_string(), "realpath".to_string(), "--canonicalize-existing".to_string(), "/tmp/no_such_file_xyzzy".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); __SH_RC.load(Ordering::Relaxed) == 0 }; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    { let __sh_operand: Vec<String> = vec!["--canonicalize-missing on a path with a non‑existent leaf".to_string(), "realpath".to_string(), "--canonicalize-missing".to_string(), "/tmp/no_such_file_xyzzy".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--canonicalize-missing on a completely imaginary path".to_string(), "realpath".to_string(), "--canonicalize-missing".to_string(), "/nonexistent/deeply/missing/file".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--canonicalize-missing on a path with .. and non‑existent parts".to_string(), "realpath".to_string(), "--canonicalize-missing".to_string(), "/tmp/../nonexistent/../foo".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--logical: /bin/..  (bin is a symlink, logical resolves .. before following it)".to_string(), "realpath".to_string(), "--logical".to_string(), "/bin/..".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["default (--physical) for comparison: /bin/..".to_string(), "realpath".to_string(), "--physical".to_string(), "/bin/..".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--physical: /bin/..  (explicit, same as default)".to_string(), "realpath".to_string(), "--physical".to_string(), "/bin/..".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--strip (no symlink expansion) on /usr/bin/vi".to_string(), "realpath".to_string(), "--strip".to_string(), "/usr/bin/vi".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--strip on /bin  (symlink /bin → usr/bin)".to_string(), "realpath".to_string(), "--strip".to_string(), "/bin".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["compare: default (--physical) on /bin".to_string(), "realpath".to_string(), "/bin".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--relative-to=/usr/bin for /usr/bin/sh".to_string(), "realpath".to_string(), "--relative-to=/usr/bin".to_string(), "/usr/bin/sh".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--relative-to=/tmp for /etc/hostname".to_string(), "realpath".to_string(), "--relative-to=/tmp".to_string(), "/etc/hostname".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--relative-to=/ for /etc/hostname".to_string(), "realpath".to_string(), "--relative-to=/".to_string(), "/etc/hostname".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--relative-base=/etc for /etc/hostname (below /etc → relative)".to_string(), "realpath".to_string(), "--relative-base=/etc".to_string(), "/etc/hostname".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--relative-base=/etc for /usr/bin/sh (not below /etc → absolute)".to_string(), "realpath".to_string(), "--relative-base=/etc".to_string(), "/usr/bin/sh".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["--relative-base=/ with --relative-to=/tmp  (combined)".to_string(), "realpath".to_string(), "--relative-base=/".to_string(), "--relative-to=/tmp".to_string(), "/etc/hostname".to_string(), "/usr/bin/sh".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    print!("==== --zero with two paths ====\n");
    print!("  $ realpath --zero /bin /usr/bin/sh\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __SH_RC.store(__sh_run("'realpath' '--zero' '/bin' '/usr/bin/sh'"), Ordering::Relaxed);
    let rc_zero = __SH_RC.load(Ordering::Relaxed).to_string();
    print!("  (raw output with NULs above; use od to verify):\n");
    print!("  ");
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_pbuf4: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", format!("{} | {}", "'realpath' '--zero' '/bin' '/usr/bin/sh'".to_string(), "'od' '-A' 'n' '-t' 'x1z'".to_string()), "'head' '-3'"), Some(&__sh_pbuf4)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    __sh_print_words(&[vec![format!("  exit code: {}\n\n", __sh_atoi(&rc_zero.clone()))]], false, false);
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_buf = { let __sh_operand: Vec<String> = vec!["--quiet suppresses error message for a truly invalid path".to_string(), "realpath".to_string(), "--quiet".to_string(), "/nonexistent_dir_xyzzy/foo".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); __SH_RC.load(Ordering::Relaxed) == 0 }; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    { let __sh_buf = { let __sh_operand: Vec<String> = vec!["without --quiet for comparison (stderr appears)".to_string(), "realpath".to_string(), "/nonexistent_dir_xyzzy/foo".to_string()]; try_(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); __SH_RC.load(Ordering::Relaxed) == 0 }; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    print!("=== All tests completed ===\n");
    __SH_RC.store(0, Ordering::Relaxed);
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn try_(p0: String, __rest: &[String]) {
    let desc = p0.clone();
    __SHV_cmd.with(|v| *v.borrow_mut() = __rest.to_vec());
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("==== {} ====", desc.clone())]], true, false);
    print!("  $ ");
    __SH_RC.store(0, Ordering::Relaxed);
    let __sh_items5 = { let __sh_value = __SHV_cmd.with(|v| v.borrow().clone()); let __sh_offset = if 0 < 0 { (__sh_value.len() as i64 + 0).max(0) } else { 0 } as usize; let __sh_offset = __sh_offset.min(__sh_value.len()); let __sh_len = if -1 < 0 { __sh_value.len() as i64 - __sh_offset as i64 } else { -1 };                          let __sh_start = __sh_offset.max(0) as usize; let __sh_stop = ((__sh_start as i64 + __sh_len).max(__sh_start as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_start);                          __sh_value[__sh_start..__sh_stop].to_vec() };
    let mut __sh_index6: usize = 0;
    loop {
    if !(__sh_index6 < __sh_items5.len()) { break; }
        __SHV_arg.with(|v| *v.borrow_mut() = __sh_items5[__sh_index6].clone());
        if { let __sh_mapped = __sh_regex(&__SHV_arg.with(|v| v.borrow().clone()).to_string(), &"[[:space:]]".to_string().to_string(), ""); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } } {
            __sh_print_words(&[vec![format!("'{}' ", __SHV_arg.with(|v| v.borrow().clone()))]], false, false);
            __SH_RC.store(0, Ordering::Relaxed);
        } else {
            __sh_print_words(&[vec![format!("{} ", __SHV_arg.with(|v| v.borrow().clone()))]], false, false);
            __SH_RC.store(0, Ordering::Relaxed);
        }
    __sh_index6 += 1;
    }
    println!();
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_stdout.with(|v| *v.borrow_mut() = { let (__sh_ch, __sh_res) = __sh_capture_rc(&format!("{} 2> {}", __sh_wq(&{ let __sh_value = __SHV_cmd.with(|v| v.borrow().clone()); let __sh_offset = if 0 < 0 { (__sh_value.len() as i64 + 0).max(0) } else { 0 } as usize; let __sh_offset = __sh_offset.min(__sh_value.len()); let __sh_len = if -1 < 0 { __sh_value.len() as i64 - __sh_offset as i64 } else { -1 };                          let __sh_start = __sh_offset.max(0) as usize; let __sh_stop = ((__sh_start as i64 + __sh_len).max(__sh_start as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_start);                          __sh_value[__sh_start..__sh_stop].to_vec() }), __sh_q(&format!("/tmp/realpath_stderr.{}", std::process::id() as i64)))); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch });
    let rc = __SH_RC.load(Ordering::Relaxed).to_string();
    __SH_RC.store(0, Ordering::Relaxed);
    let stderr = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    { let __sh_buf = {
__SH_RC.store(__sh_spawn(&format!("{} 2> {}", __sh_wq(&vec!["cat".to_string(), format!("/tmp/realpath_stderr.{}", std::process::id() as i64)]), __sh_q("/dev/null")), None), Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{}", __e) } };
        for __sh_path in &vec![format!("/tmp/realpath_stderr.{}", std::process::id() as i64)] {
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
    if __SHV_stdout.with(|v| v.borrow().clone()) != "".to_string() {
        if __sh_fnmatch(&format!("*{}*", "\x00"), &__SHV_stdout.with(|v| v.borrow().clone())) {
            print!("  stdout (NUL‑terminated): ");
            __SH_RC.store(0, Ordering::Relaxed);
            let mut __sh_pbuf7: Vec<u8> = Vec::new();
            __SH_PIPESTATUS.lock().unwrap().clear();
            {
    let __sh_oldout8 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_print_words(&[vec![__SHV_stdout.with(|v| v.borrow().clone())]], false, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf7 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout8);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
            __SH_RC.store(__sh_spawn("'od' '-A' 'n' '-t' 'x1z'", Some(&__sh_pbuf7)), Ordering::Relaxed);
            __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
            print!("\n");
            __SH_RC.store(0, Ordering::Relaxed);
        } else {
            __sh_print_words(&[vec![format!("  stdout: {}\n", __SHV_stdout.with(|v| v.borrow().clone()))]], false, false);
            __SH_RC.store(0, Ordering::Relaxed);
        }
    } else {
        print!("  stdout: (empty)\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    if stderr.clone() != "".to_string() {
        __sh_print_words(&[vec![format!("  stderr: {}\n", stderr.clone())]], false, false);
        __SH_RC.store(0, Ordering::Relaxed);
    } else {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    __sh_print_words(&[vec![format!("  exit code: {}\n\n", __sh_atoi(&rc.clone()))]], false, false);
    __SH_RC.store(0, Ordering::Relaxed);
}