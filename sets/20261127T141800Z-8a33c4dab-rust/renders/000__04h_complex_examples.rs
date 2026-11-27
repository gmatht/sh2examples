use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());
static __SH_TRAPS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp0: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp1: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_d: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_file: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_files: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

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

fn __sh_glob(pat: &str) -> Vec<String> {
    let cmd = format!("printf '%s\\n' {pat}");
    let out = __sh_cap_bytes(&cmd, None);
    let s = String::from_utf8_lossy(&out);
    let v: Vec<String> = s.lines().map(|x| x.to_string()).collect();
    if v.is_empty() { vec![pat.to_string()] } else { v }
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
    __SHV_d.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(true, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    { let __sh_buf = { let __sh_dir = __SHV_d.with(|v| v.borrow().clone()); let __sh_res = std::env::set_current_dir(&__sh_dir); if __sh_res.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __sh_res.is_ok() { 0 } else { 1 }, Ordering::Relaxed); __sh_res.is_ok() }; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        __sh_run_traps();
        std::process::exit(1);
    }
    }
    print!("=== Complex Backtick Examples ===\n");
    __SH_RC.store(0, Ordering::Relaxed);
    let nested_result = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![format!("Three wells: {}", "well\nwell\nwell")]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    println!("Nested backticks: {}", nested_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let count = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf0: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'ls' '-1'", "'wc' '-l'"), Some(&__sh_pbuf0)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    println!("File count: {}", count.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    if "root" == "root" {
        print!("Running as root\n");
    } else {
        print!("Not running as root\n");
    }
    print!("Running on macOS\n");
    get_file_size("000__01_file_directory_operations.sh".to_string());
    __SHV_files.with(|v| *v.borrow_mut() = { let __sh_split_s: String = { let (__sh_ch, __sh_res) = __sh_capture_rc(&format!("{} 2> {}", __sh_wq(&__sh_cat(&[vec!["ls".to_string()], vec!["-1".to_string()], __sh_glob("*.sh"), __sh_glob("examples/*.sh")])), __sh_q("/dev/null"))); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch }; __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() });
    __SH_RC.store(0, Ordering::Relaxed);
    println!("Shell scripts found: {}", __SHV_files.with(|v| v.borrow().len() as i64).to_string());
    let __sh_items1 = { let __sh_value = __SHV_files.with(|v| v.borrow().clone()); let __sh_offset = if 0 < 0 { (__sh_value.len() as i64 + 0).max(0) } else { 0 } as usize; let __sh_offset = __sh_offset.min(__sh_value.len()); let __sh_len = if -1 < 0 { __sh_value.len() as i64 - __sh_offset as i64 } else { -1 };                          let __sh_start = __sh_offset.max(0) as usize; let __sh_stop = ((__sh_start as i64 + __sh_len).max(__sh_start as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_start);                          __sh_value[__sh_start..__sh_stop].to_vec() };
    let mut __sh_index2: usize = 0;
    loop {
    if !(__sh_index2 < __sh_items1.len()) { break; }
        __SHV_file.with(|v| *v.borrow_mut() = __sh_items1[__sh_index2].clone());
        println!("  - {}", __SHV_file.with(|v| v.borrow().clone()));
        __SH_RC.store(0, Ordering::Relaxed);
    __sh_index2 += 1;
    }
    let _ = std::fs::write("file1.txt", "apple\nbanana\ncherry\n");
    let _ = std::fs::write("file2.txt", "banana\ncherry\ndate\n");
    __SH_RC.store(0, Ordering::Relaxed);
    let process_result = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    {
        __SHV___ps_tmp0.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
        __SH_RC.store(__sh_spawn(&format!("{} > {}", "'sort' 'file1.txt'", __sh_q(&__SHV___ps_tmp0.with(|v| v.borrow().clone()))), None), Ordering::Relaxed);
        __SHV___ps_tmp1.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, ""); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
        __SH_RC.store(__sh_spawn(&format!("{} > {}", "'sort' 'file2.txt'", __sh_q(&__SHV___ps_tmp1.with(|v| v.borrow().clone()))), None), Ordering::Relaxed);
        __SH_RC.store(__sh_run(&__sh_wq(&vec!["comm".to_string(), "-23".to_string(), __SHV___ps_tmp0.with(|v| v.borrow().clone()), __SHV___ps_tmp1.with(|v| v.borrow().clone())])), Ordering::Relaxed);
        if __SH_RC.load(Ordering::Relaxed) == 0 {
            {
                let mut __sh_status: i32 = 0;
                let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
                    std::io::ErrorKind::NotFound => "No such file or directory".into(),
                    std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
                    std::io::ErrorKind::AlreadyExists => "File exists".into(),
                    std::io::ErrorKind::IsADirectory => "Is a directory".into(),
                    std::io::ErrorKind::NotADirectory => "Not a directory".into(),
                    _ => format!("{__e}") } };
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
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    print!("Process substitution result:\n");
    println!("{}", process_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let here_string_result = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __SH_RC.store(__sh_spawn("'tr' 'a-z' 'A-Z'", Some(format!("{}\n", "hello world").as_bytes())), Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    println!("Here string result: {}", here_string_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let perl_result = { let (__sh_ch, __sh_res) = __sh_capture_rc("'perl' '-e' 'print \"Hello from Perl\\n\"'"); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch };
    println!("Perl result: {}", perl_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{__e}") } };
        for __sh_path in &vec!["file1.txt".to_string(), "file2.txt".to_string()] {
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
    print!("=== Complex Backtick Examples Complete ===\n");
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_dir = "/".to_string(); let __sh_res = std::env::set_current_dir(&__sh_dir); if __sh_res.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __sh_res.is_ok() { 0 } else { 1 }, Ordering::Relaxed); };
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{__e}") } };
        for __sh_path in &vec![__SHV_d.with(|v| v.borrow().clone())] {
            match std::fs::symlink_metadata(__sh_path) {
                Ok(__sh_mapped) => {
            let __sh_res = if __sh_mapped.is_dir() {
                std::fs::remove_dir_all(__sh_path)
            } else {
                std::fs::remove_file(__sh_path)
            };
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
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn get_file_size(p0: String) {
    let __sh_loc_file: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
    __SHV_file.with(|v| *v.borrow_mut() = format!("{}", p0.clone()));
    __SH_RC.store(0, Ordering::Relaxed);
    let size = { let __sh_split_s: String = { let (__sh_ch, __sh_res) = __sh_capture_rc(&format!("{} < {}", "'wc' '-c'", __sh_q(&__SHV_file.with(|v| v.borrow().clone())))); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch }; __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }.join(" ");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("File {} has {} bytes", __SHV_file.with(|v| v.borrow().clone()), size.clone())]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    match __sh_loc_file.take() { Some(__sh_loc) => { __SHV_file.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
}