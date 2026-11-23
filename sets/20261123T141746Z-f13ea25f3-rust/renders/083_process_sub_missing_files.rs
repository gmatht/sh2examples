use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp0: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp1: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn __sh_wq(ws: &[String]) -> String {
    ws.iter().map(|w| __sh_q(w)).collect::<Vec<_>>().join(" ")
}

fn __sh_puts(s: &str) {
    __sh_sink(s.as_bytes());
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
    print!("start\n");
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_buf = {
{
    __SHV___ps_tmp0.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp0.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        __sh_puts("a\n");
        __SH_RC.store(0, Ordering::Relaxed);
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    __SHV___ps_tmp1.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp1.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        __sh_puts("b\n");
        __SH_RC.store(0, Ordering::Relaxed);
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    __SH_RC.store(__sh_run(&__sh_wq(&vec!["diff".to_string(), __SHV___ps_tmp0.with(|v| v.borrow().clone()), __SHV___ps_tmp1.with(|v| v.borrow().clone())])), Ordering::Relaxed);
    if __SH_RC.load(Ordering::Relaxed) == 0 {
        {
            let mut __sh_status: i32 = 0;
            let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
                std::io::ErrorKind::NotFound => "No such file or directory".into(),
                std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
                std::io::ErrorKind::AlreadyExists => "File exists".into(),
                std::io::ErrorKind::IsADirectory => "Is a directory".into(),
                std::io::ErrorKind::NotADirectory => "Not a directory".into(),
                _ => format!("{}", __e) } };
            for __sh_path in &vec![__SHV___ps_tmp0.with(|v| v.borrow().clone()), __SHV___ps_tmp1.with(|v| v.borrow().clone())] {
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
    }
}
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    print!("end\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("done: {}", __SH_RC.load(Ordering::Relaxed));
    __SH_RC.store(0, Ordering::Relaxed);
    std::process::ExitCode::SUCCESS
}