use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

fn __sh_cap_bytes(cmd: &str, input: Option<&[u8]>) -> Vec<u8> {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    if input.is_some() { c.stdin(std::process::Stdio::piped()); }
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return Vec::new() };
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

fn __sh_glob(pat: &str) -> Vec<String> {
    let cmd = format!("printf '%s\\n' {}", pat);
    let out = __sh_cap_bytes(&cmd, None);
    let s = String::from_utf8_lossy(&out);
    let v: Vec<String> = s.lines().map(|x| x.to_string()).collect();
    if v.is_empty() { vec![pat.to_string()] } else { v }
}

fn main() {
    print!("== Basic brace expansion ==\n");
    print!("1 2 3 4 5\n");
    print!("a b c\n");
    print!("00 02 04\n");
    print!("== Advanced brace expansion ==\n");
    print!("a1 a2 a3 b1 b2 b3 c1 c2 c3\n");
    print!("1 3 5 7 9\n");
    print!("a d g j m p s v y\n");
    print!("== Practical examples ==\n");
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("touch").arg("file_001.txt").arg("file_002.txt").arg("file_003.txt").arg("file_004.txt").arg("file_005.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_RC.store(__sh_spawn(&{ let mut __o = String::new(); for (__i, __w) in {&[vec!["ls".to_string()], __sh_glob("file_*.txt")].concat()}.iter().enumerate() { if __i > 0 { __o.push(' '); } __o.push_str(&{ let __t: &str = {__w}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }); } __o }, None), Ordering::SeqCst);
    __SH_RC.store(__sh_spawn(&{ let mut __o = String::new(); for (__i, __w) in {&[vec!["rm".to_string()], __sh_glob("file_*.txt")].concat()}.iter().enumerate() { if __i > 0 { __o.push(' '); } __o.push_str(&{ let __t: &str = {__w}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }); } __o }, None), Ordering::SeqCst);
    println!("{}", format!("brace_expand: {}", "1 2 3 4 5".to_string()));
}