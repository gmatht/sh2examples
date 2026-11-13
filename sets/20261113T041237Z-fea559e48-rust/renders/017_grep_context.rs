use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_d: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

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

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    let mut __sh_pbuf0: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"line1\nline2\nTARGET\nline4\nline5\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf0 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("-A").arg("2").arg("TARGET").stdin(std::process::Stdio::piped()).spawn() { Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf0); } __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1) }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    let mut __sh_pbuf2: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"line1\nline2\nTARGET\nline4\nline5\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf2 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("-B").arg("2").arg("TARGET").stdin(std::process::Stdio::piped()).spawn() { Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf2); } __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1) }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    let mut __sh_pbuf4: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"line1\nline2\nTARGET\nline4\nline5\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf4 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("-C").arg("1").arg("TARGET").stdin(std::process::Stdio::piped()).spawn() { Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf4); } __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1) }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    __SHV_d.with(|v| *v.borrow_mut() = { let (__c, __r) = { let __sc = std::process::Command::new("mktemp").arg("-d").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c });
    if !{ let __d = __SHV_d.with(|v| v.borrow().clone()); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() } {
        std::process::exit(1);
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("temp_file1.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "pattern in file1\n".as_bytes());
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("temp_file2.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "no pattern in file2\n".as_bytes());
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("temp_file3.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "pattern in file3\n".as_bytes());
    }
    println!("{}", "Recursive search results:".to_string());
    __SH_RC.store(__sh_spawn(&{ let mut __o = String::new(); for (__i, __w) in {&[vec!["grep".to_string()], vec!["-r".to_string()], vec!["pattern".to_string()], vec![".".to_string()], __sh_glob("--include=*.txt")].concat()}.iter().enumerate() { if __i > 0 { __o.push(' '); } __o.push_str(&{ let __t: &str = {__w}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }); } __o }, None), Ordering::SeqCst);
    println!("{}", "Result 2...".to_string());
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", { let mut __o = String::new(); for (__i, __w) in {&[vec!["grep".to_string()], vec!["-l".to_string()], vec!["pattern".to_string()], __sh_glob("*.txt")].concat()}.iter().enumerate() { if __i > 0 { __o.push(' '); } __o.push_str(&{ let __t: &str = {__w}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }); } __o }, "'sort'".to_string()), Some(&[])), Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    println!("{}", "Result 3...".to_string());
    __SH_RC.store(__sh_spawn(&{ let mut __o = String::new(); for (__i, __w) in {&[vec!["grep".to_string()], vec!["-L".to_string()], vec!["pattern".to_string()], __sh_glob("*.txt")].concat()}.iter().enumerate() { if __i > 0 { __o.push(' '); } __o.push_str(&{ let __t: &str = {__w}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }); } __o }, None), Ordering::SeqCst);
    let _ = { let __d = "/".to_string(); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() };
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-r").arg("-f").arg(__SHV_d.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    let matched = {
    let (__c, __r) = { let __sc = std::process::Command::new("grep").arg("-c").arg(".*").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __sc { Ok(mut __sc) => { if let Some(mut __ci) = __sc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, format!("{}\n", "test".to_string()).as_bytes()); } let __so = __sc.wait_with_output(); match __so { Ok(__so) => { let mut __ss = String::from_utf8_lossy(&__so.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __so.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
};
    println!("{}", format!("  grep_exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    println!("{}", format!("  match_count: {}", matched.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
}