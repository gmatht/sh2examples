use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_TTY_DEV: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_dev: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_tmp_stderr: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_tmp_stdout: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

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

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_TTY_DEV.with(|v| *v.borrow_mut() = "".to_string());
    for dev in ["/dev/pts/2", "/dev/pts/3", "/dev/pts/4"] {
        __SHV_dev.with(|v| *v.borrow_mut() = dev.to_string());
        if std::fs::File::open(&dev).is_ok() {
            __SHV_TTY_DEV.with(|v| *v.borrow_mut() = dev.to_string());
            break;
        } else {
        }
    }
    if !(__SHV_TTY_DEV.with(|v| v.borrow().clone()) != "".to_string()) {
        for dev in ["\x01SH2GLOB\x01/dev/pts/*"] {
            if std::fs::File::open(&dev).is_ok() {
                __SHV_TTY_DEV.with(|v| *v.borrow_mut() = dev.to_string());
                break;
            } else {
            }
        }
    } else {
    }
    println!("{}", format!("Using terminal device: {}", { let __v = __SHV_TTY_DEV.with(|v| v.borrow().clone()); if __v.is_empty() { "NONE".to_string() } else { __v.to_string() } }));
    println!("{}", "".to_string());
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 1: tty (default) — with a real terminal".to_string());
    println!("{}", "============================================================".to_string());
    if __SHV_TTY_DEV.with(|v| v.borrow().clone()) != "".to_string() {
        {
            let __oldin = __SH_STDIN.lock().unwrap().take();
            let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
            *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV_TTY_DEV.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
            *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV_TTY_DEV.with(|v| v.borrow().clone())));
            { let __a: Vec<String> = vec!["01-default-terminal".to_string(), "tty".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
            *__SH_STDIN.lock().unwrap() = __oldin;
            *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
        }
    } else {
        println!("{}", "  (skipped — no terminal device available)".to_string());
        println!("{}", "".to_string());
    }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 2: tty (default) — stdin from /dev/null".to_string());
    println!("{}", "============================================================".to_string());
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/dev/null").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/dev/null"));
        { let __a: Vec<String> = vec!["02-not-a-tty".to_string(), "tty".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 3: tty (default) — piped input".to_string());
    println!("{}", "============================================================".to_string());
    { let __a: Vec<String> = vec!["03-pipe-notty".to_string(), "bash".to_string(), "-c".to_string(), "echo \"dummy\" | tty".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 4: tty -s — with a real terminal (silent)".to_string());
    println!("{}", "============================================================".to_string());
    if __SHV_TTY_DEV.with(|v| v.borrow().clone()) != "".to_string() {
        {
            let __oldin = __SH_STDIN.lock().unwrap().take();
            let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
            *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV_TTY_DEV.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
            *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV_TTY_DEV.with(|v| v.borrow().clone())));
            { let __a: Vec<String> = vec!["04-silent-terminal".to_string(), "tty".to_string(), "-s".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
            *__SH_STDIN.lock().unwrap() = __oldin;
            *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
        }
    } else {
        println!("{}", "  (skipped — no terminal device available)".to_string());
        println!("{}", "".to_string());
    }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 5: tty -s — stdin from /dev/null (silent, not a tty)".to_string());
    println!("{}", "============================================================".to_string());
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/dev/null").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/dev/null"));
        { let __a: Vec<String> = vec!["05-silent-notty".to_string(), "tty".to_string(), "-s".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 6: tty -s — piped input (silent, not a tty)".to_string());
    println!("{}", "============================================================".to_string());
    { let __a: Vec<String> = vec!["06-silent-pipe".to_string(), "bash".to_string(), "-c".to_string(), "echo \"dummy\" | tty -s".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 7: tty --silent — long form, with a real terminal".to_string());
    println!("{}", "============================================================".to_string());
    if __SHV_TTY_DEV.with(|v| v.borrow().clone()) != "".to_string() {
        {
            let __oldin = __SH_STDIN.lock().unwrap().take();
            let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
            *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV_TTY_DEV.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
            *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV_TTY_DEV.with(|v| v.borrow().clone())));
            { let __a: Vec<String> = vec!["07-long-silent".to_string(), "tty".to_string(), "--silent".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
            *__SH_STDIN.lock().unwrap() = __oldin;
            *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
        }
    } else {
        println!("{}", "  (skipped — no terminal device available)".to_string());
        println!("{}", "".to_string());
    }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 8: tty --quiet — long form, with a real terminal".to_string());
    println!("{}", "============================================================".to_string());
    if __SHV_TTY_DEV.with(|v| v.borrow().clone()) != "".to_string() {
        {
            let __oldin = __SH_STDIN.lock().unwrap().take();
            let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
            *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV_TTY_DEV.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
            *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV_TTY_DEV.with(|v| v.borrow().clone())));
            { let __a: Vec<String> = vec!["08-long-quiet".to_string(), "tty".to_string(), "--quiet".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
            *__SH_STDIN.lock().unwrap() = __oldin;
            *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
        }
    } else {
        println!("{}", "  (skipped — no terminal device available)".to_string());
        println!("{}", "".to_string());
    }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 9: tty --silent — stdin from /dev/null (not a tty)".to_string());
    println!("{}", "============================================================".to_string());
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/dev/null").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/dev/null"));
        { let __a: Vec<String> = vec!["09-long-silent-notty".to_string(), "tty".to_string(), "--silent".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 10: tty --quiet — stdin from /dev/null (not a tty)".to_string());
    println!("{}", "============================================================".to_string());
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/dev/null").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/dev/null"));
        { let __a: Vec<String> = vec!["10-long-quiet-notty".to_string(), "tty".to_string(), "--quiet".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 11: tty (default) — inheriting stdin from the script".to_string());
    println!("{}", "============================================================".to_string());
    { let __a: Vec<String> = vec!["11-inherited".to_string(), "tty".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 12: tty -s — inheriting stdin from the script".to_string());
    println!("{}", "============================================================".to_string());
    { let __a: Vec<String> = vec!["12-inherited-silent".to_string(), "tty".to_string(), "-s".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 13: tty -s -s — repeated silent flag".to_string());
    println!("{}", "============================================================".to_string());
    if __SHV_TTY_DEV.with(|v| v.borrow().clone()) != "".to_string() {
        {
            let __oldin = __SH_STDIN.lock().unwrap().take();
            let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
            *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV_TTY_DEV.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
            *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV_TTY_DEV.with(|v| v.borrow().clone())));
            { let __a: Vec<String> = vec!["13-double-silent".to_string(), "tty".to_string(), "-s".to_string(), "-s".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
            *__SH_STDIN.lock().unwrap() = __oldin;
            *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
        }
    } else {
        println!("{}", "  (skipped — no terminal device available)".to_string());
        println!("{}", "".to_string());
    }
    println!("{}", "".to_string());
    println!("{}", "All tty demo sections completed.".to_string());
}

fn __sh_fn_capture(p0: String, __rest: &[String]) {
    let label = p0.clone();
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_tmp_stdout.with(|v| *v.borrow_mut() = { let (__c, __r) = { let __sc = std::process::Command::new("mktemp").arg("/tmp/tty_demo_stdout_XXXXXX").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c });
    __SHV_tmp_stderr.with(|v| *v.borrow_mut() = { let (__c, __r) = { let __sc = std::process::Command::new("mktemp").arg("/tmp/tty_demo_stderr_XXXXXX").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c });
    __SH_RC.store(__sh_spawn(&format!("{} 2> {}", format!("{} > {}", { let mut __o = String::new(); for (__i, __w) in {&[__rest.to_vec()].concat()}.iter().enumerate() { if __i > 0 { __o.push(' '); } __o.push_str(&{ let __t: &str = {__w}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }); } __o }, { let __t: &str = {&__SHV_tmp_stdout.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }), { let __t: &str = {&__SHV_tmp_stderr.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }), None), Ordering::SeqCst);
    let ec = { let __si = __SH_RC.load(Ordering::SeqCst).to_string(); let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }.join(" ");
    __SH_RC.store(0, Ordering::SeqCst);
    let so = { let (__c, __r) = { let __sc = std::process::Command::new("cat").arg(__SHV_tmp_stdout.with(|v| v.borrow().clone())).stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c };
    __SH_RC.store(0, Ordering::SeqCst);
    let se = { let (__c, __r) = { let __sc = std::process::Command::new("cat").arg(__SHV_tmp_stderr.with(|v| v.borrow().clone())).stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c };
    __SH_RC.store(__sh_spawn(&format!("{}{}", "'rm' '-f' ", vec![{ let __t: &str = {&__SHV_tmp_stdout.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }, { let __t: &str = {&__SHV_tmp_stderr.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }].join(" ")), None), Ordering::SeqCst);
    println!("{}", format!("--- [{}] ---", label.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("  cmd     : {}", __rest.to_vec().join(" ")));
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("  exitcode: {}", ec.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    if so.clone() != "".to_string() {
        println!("{}", format!("  stdout  : {}", so.clone()));
        __SH_RC.store(0, Ordering::SeqCst);
    } else {
        print!("{}", "  stdout  : (empty)\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    if se.clone() != "".to_string() {
        println!("{}", format!("  stderr  : {}", se.clone()));
        __SH_RC.store(0, Ordering::SeqCst);
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    print!("{}", "\n");
    __SH_RC.store(0, Ordering::SeqCst);
    __SH_RC.store((ec.clone().trim().parse::<i64>().unwrap_or(0)) as i32, Ordering::SeqCst); return;
}