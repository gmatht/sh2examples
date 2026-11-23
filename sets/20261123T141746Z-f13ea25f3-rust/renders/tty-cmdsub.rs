use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_TTY_DEV: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_dev: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_tmp_stderr: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_tmp_stdout: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn __sh_wq(ws: &[String]) -> String {
    ws.iter().map(|w| __sh_q(w)).collect::<Vec<_>>().join(" ")
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

fn __sh_glob(pat: &str) -> Vec<String> {
    let cmd = format!("printf '%s\\n' {}", pat);
    let out = __sh_cap_bytes(&cmd, None);
    let s = String::from_utf8_lossy(&out);
    let v: Vec<String> = s.lines().map(|x| x.to_string()).collect();
    if v.is_empty() { vec![pat.to_string()] } else { v }
}

fn __sh_uid() -> u32 {
    std::fs::read_to_string("/proc/self/status").ok()
        .and_then(|s| s.lines().find(|l| l.starts_with("Uid:"))
            .and_then(|l| l.split_whitespace().nth(1)).and_then(|v| v.parse().ok()))
        .unwrap_or(0)
}

fn __sh_gid() -> u32 {
    std::fs::read_to_string("/proc/self/status").ok()
        .and_then(|s| s.lines().find(|l| l.starts_with("Gid:"))
            .and_then(|l| l.split_whitespace().nth(1)).and_then(|v| v.parse().ok()))
        .unwrap_or(0)
}

fn __sh_access(p: &str, want: u32) -> bool {
    use std::os::unix::fs::MetadataExt;
    let md = match std::fs::metadata(p) { Ok(m) => m, Err(_) => return false };
    if __sh_uid() == 0 {
        return want & 1 == 0 || md.mode() & 0o111 != 0;
    }
    let mode = md.mode();
    let bits = if md.uid() == __sh_uid() {
        (mode >> 6) & 7
    } else if md.gid() == __sh_gid() {
        (mode >> 3) & 7
    } else {
        mode & 7
    };
    bits & want != 0
}

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
}

fn __sh_catf(argv: &[String]) -> Option<String> {
    let mut out = String::new();
    for p in argv {
        match std::fs::read(p) {
            Ok(d) => out.push_str(&String::from_utf8_lossy(&d)),
            Err(_) => return None,
        }
    }
    while out.ends_with('\n') { out.pop(); }
    Some(out)
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
    __SHV_TTY_DEV.with(|v| *v.borrow_mut() = "".to_string());
    for _dev in ["/dev/pts/2", "/dev/pts/3", "/dev/pts/4"] {
        __SHV_dev.with(|v| *v.borrow_mut() = _dev.to_string());
        if __sh_access(&_dev.to_string(), 4) {
            __SHV_TTY_DEV.with(|v| *v.borrow_mut() = _dev.to_string());
            break;
        } else {
        }
    }
    if __SHV_TTY_DEV.with(|v| v.borrow().clone()) == "".to_string() {
        let __sh_items1 = __sh_glob("/dev/pts/*");
        let mut __sh_index2: usize = 0;
        loop {
        if !(__sh_index2 < __sh_items1.len()) { break; }
            __SHV_dev.with(|v| *v.borrow_mut() = __sh_items1[__sh_index2].clone());
            if __sh_access(&__SHV_dev.with(|v| v.borrow().clone()), 4) {
                __SHV_TTY_DEV.with(|v| *v.borrow_mut() = __SHV_dev.with(|v| v.borrow().clone()));
                break;
            } else {
            }
        __sh_index2 += 1;
        }
    } else {
    }
    println!("Using terminal device: {}", { let __sh_value = __SHV_TTY_DEV.with(|v| v.borrow().clone()); if __sh_value.is_empty() { "NONE".to_string() } else { __sh_value.to_string() } });
    println!();
    print!("============================================================\n");
    print!(" SECTION 1: tty (default) — with a real terminal\n");
    print!("============================================================\n");
    if __SHV_TTY_DEV.with(|v| v.borrow().clone()) != "".to_string() {
        {
            let __oldin = __SH_STDIN.lock().unwrap().take();
            let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
            *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV_TTY_DEV.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
            *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV_TTY_DEV.with(|v| v.borrow().clone())));
            { let __sh_operand: Vec<String> = vec!["01-default-terminal".to_string(), "tty".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
            *__SH_STDIN.lock().unwrap() = __oldin;
            *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
        }
    } else {
        print!("  (skipped — no terminal device available)\n");
        println!();
    }
    print!("============================================================\n");
    print!(" SECTION 2: tty (default) — stdin from /dev/null\n");
    print!("============================================================\n");
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/dev/null").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/dev/null"));
        { let __sh_operand: Vec<String> = vec!["02-not-a-tty".to_string(), "tty".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    print!("============================================================\n");
    print!(" SECTION 3: tty (default) — piped input\n");
    print!("============================================================\n");
    { let __sh_operand: Vec<String> = vec!["03-pipe-notty".to_string(), "bash".to_string(), "-c".to_string(), "echo \"dummy\" | tty".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    print!("============================================================\n");
    print!(" SECTION 4: tty -s — with a real terminal (silent)\n");
    print!("============================================================\n");
    if __SHV_TTY_DEV.with(|v| v.borrow().clone()) != "".to_string() {
        {
            let __oldin = __SH_STDIN.lock().unwrap().take();
            let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
            *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV_TTY_DEV.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
            *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV_TTY_DEV.with(|v| v.borrow().clone())));
            { let __sh_operand: Vec<String> = vec!["04-silent-terminal".to_string(), "tty".to_string(), "-s".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
            *__SH_STDIN.lock().unwrap() = __oldin;
            *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
        }
    } else {
        print!("  (skipped — no terminal device available)\n");
        println!();
    }
    print!("============================================================\n");
    print!(" SECTION 5: tty -s — stdin from /dev/null (silent, not a tty)\n");
    print!("============================================================\n");
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/dev/null").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/dev/null"));
        { let __sh_operand: Vec<String> = vec!["05-silent-notty".to_string(), "tty".to_string(), "-s".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    print!("============================================================\n");
    print!(" SECTION 6: tty -s — piped input (silent, not a tty)\n");
    print!("============================================================\n");
    { let __sh_operand: Vec<String> = vec!["06-silent-pipe".to_string(), "bash".to_string(), "-c".to_string(), "echo \"dummy\" | tty -s".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    print!("============================================================\n");
    print!(" SECTION 7: tty --silent — long form, with a real terminal\n");
    print!("============================================================\n");
    if __SHV_TTY_DEV.with(|v| v.borrow().clone()) != "".to_string() {
        {
            let __oldin = __SH_STDIN.lock().unwrap().take();
            let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
            *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV_TTY_DEV.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
            *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV_TTY_DEV.with(|v| v.borrow().clone())));
            { let __sh_operand: Vec<String> = vec!["07-long-silent".to_string(), "tty".to_string(), "--silent".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
            *__SH_STDIN.lock().unwrap() = __oldin;
            *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
        }
    } else {
        print!("  (skipped — no terminal device available)\n");
        println!();
    }
    print!("============================================================\n");
    print!(" SECTION 8: tty --quiet — long form, with a real terminal\n");
    print!("============================================================\n");
    if __SHV_TTY_DEV.with(|v| v.borrow().clone()) != "".to_string() {
        {
            let __oldin = __SH_STDIN.lock().unwrap().take();
            let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
            *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV_TTY_DEV.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
            *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV_TTY_DEV.with(|v| v.borrow().clone())));
            { let __sh_operand: Vec<String> = vec!["08-long-quiet".to_string(), "tty".to_string(), "--quiet".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
            *__SH_STDIN.lock().unwrap() = __oldin;
            *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
        }
    } else {
        print!("  (skipped — no terminal device available)\n");
        println!();
    }
    print!("============================================================\n");
    print!(" SECTION 9: tty --silent — stdin from /dev/null (not a tty)\n");
    print!("============================================================\n");
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/dev/null").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/dev/null"));
        { let __sh_operand: Vec<String> = vec!["09-long-silent-notty".to_string(), "tty".to_string(), "--silent".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    print!("============================================================\n");
    print!(" SECTION 10: tty --quiet — stdin from /dev/null (not a tty)\n");
    print!("============================================================\n");
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/dev/null").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/dev/null"));
        { let __sh_operand: Vec<String> = vec!["10-long-quiet-notty".to_string(), "tty".to_string(), "--quiet".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    print!("============================================================\n");
    print!(" SECTION 11: tty (default) — inheriting stdin from the script\n");
    print!("============================================================\n");
    { let __sh_operand: Vec<String> = vec!["11-inherited".to_string(), "tty".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    print!("============================================================\n");
    print!(" SECTION 12: tty -s — inheriting stdin from the script\n");
    print!("============================================================\n");
    { let __sh_operand: Vec<String> = vec!["12-inherited-silent".to_string(), "tty".to_string(), "-s".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    print!("============================================================\n");
    print!(" SECTION 13: tty -s -s — repeated silent flag\n");
    print!("============================================================\n");
    if __SHV_TTY_DEV.with(|v| v.borrow().clone()) != "".to_string() {
        {
            let __oldin = __SH_STDIN.lock().unwrap().take();
            let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
            *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV_TTY_DEV.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
            *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV_TTY_DEV.with(|v| v.borrow().clone())));
            { let __sh_operand: Vec<String> = vec!["13-double-silent".to_string(), "tty".to_string(), "-s".to_string(), "-s".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
            *__SH_STDIN.lock().unwrap() = __oldin;
            *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
        }
    } else {
        print!("  (skipped — no terminal device available)\n");
        println!();
    }
    println!();
    print!("All tty demo sections completed.\n");
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn __sh_fn_capture(p0: String, __rest: &[String]) {
    let label = p0.clone();
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_tmp_stdout.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, "/tmp/tty_demo_stdout_XXXXXX"); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    __SHV_tmp_stderr.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, "/tmp/tty_demo_stderr_XXXXXX"); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    __SH_RC.store(__sh_spawn(&format!("{} 2> {}", format!("{} > {}", __sh_wq(&__rest.to_vec()), __sh_q(&__SHV_tmp_stdout.with(|v| v.borrow().clone()))), __sh_q(&__SHV_tmp_stderr.with(|v| v.borrow().clone()))), None), Ordering::Relaxed);
    let ec = ({ let __sh_split_s: String = __SH_RC.load(Ordering::Relaxed).to_string(); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }).join(" ");
    let so = { let __sh_ch = __sh_catf(&vec![__SHV_tmp_stdout.with(|v| v.borrow().clone())]); __SH_RC.store(if __sh_ch.is_some() { 0 } else { 1 }, Ordering::Relaxed); __sh_ch.unwrap_or_default() };
    let se = { let __sh_ch = __sh_catf(&vec![__SHV_tmp_stderr.with(|v| v.borrow().clone())]); __SH_RC.store(if __sh_ch.is_some() { 0 } else { 1 }, Ordering::Relaxed); __sh_ch.unwrap_or_default() };
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{}", __e) } };
        for __sh_path in &vec![__SHV_tmp_stdout.with(|v| v.borrow().clone()), __SHV_tmp_stderr.with(|v| v.borrow().clone())] {
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
    println!("--- [{}] ---", label.clone());
    println!("  cmd     : {}", __rest.to_vec().join(" "));
    println!("  exitcode: {}", ec.clone());
    if so.clone() != "".to_string() {
        println!("  stdout  : {}", so.clone());
    } else {
        println!("{}", "  stdout  : (empty)");
    }
    if se.clone() != "".to_string() {
        println!("  stderr  : {}", se.clone());
    } else {
    }
    println!("{}", "");
    let __sh_value = ec.clone(); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed); return;
}