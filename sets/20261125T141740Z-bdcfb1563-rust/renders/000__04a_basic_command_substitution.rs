use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn __sh_print_words(ws: &[Vec<String>], nl: bool, _esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    if nl { s.push('\n'); }
    __sh_sink(s.as_bytes());
}

fn __sh_capture_rc_argv(argv: &[&str]) -> (String, i32) {
    if argv.is_empty() { return (String::new(), 0); }
    let mut c = std::process::Command::new(argv[0]);
    c.args(&argv[1..]);
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

fn __sh_basename_args(argv: &[String]) -> Option<String> {
    if argv.is_empty() {
        eprintln!("basename: missing operand\nTry 'basename --help' for more information.");
        return None;
    }
    if argv.len() > 2 {
        eprintln!("basename: extra operand '{}'\nTry 'basename --help' for more information.", argv[2]);
        return None;
    }
    let p = &argv[0];
    let body = p.trim_end_matches('/');
    let mut out = if p.is_empty() {
        String::new()
    } else if body.is_empty() {
        "/".to_string()
    } else {
        match body.rfind('/') { Some(i) => body[i + 1..].to_string(), None => body.to_string() }
    };
    if let Some(suffix) = argv.get(1) {
        if !suffix.is_empty() && out.len() > suffix.len() && out.ends_with(suffix.as_str()) {
            out.truncate(out.len() - suffix.len());
        }
    }
    Some(out)
}

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

fn main() -> std::process::ExitCode {
    print!("=== Basic Command Substitution ===\n");
    println!("Current date: {}", { let (__sh_ch, __sh_res) = __sh_capture_rc_argv(&["date", "+%Y"]); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch });
    println!("Current directory: {}", match __sh_basename_args(&vec![{ let __sh_split_s: String = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()]], true, false);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
}; __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }.join(" ")]) { Some(__sh_ch) => { __SH_RC.store(0, Ordering::Relaxed); __sh_ch } None => { __SH_RC.store(1, Ordering::Relaxed); String::new() } });
    let current_date = { let (__sh_ch, __sh_res) = __sh_capture_rc_argv(&["date", "+%Y%m"]); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch };
    let current_dir = match __sh_basename_args(&vec![{ let __sh_split_s: String = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()]], true, false);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
}; __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }.join(" ")]) { Some(__sh_ch) => { __SH_RC.store(0, Ordering::Relaxed); __sh_ch } None => { __SH_RC.store(1, Ordering::Relaxed); String::new() } };
    __sh_print_words(&[vec![format!("Stored date: {}", current_date.clone())]], true, false);
    __sh_print_words(&[vec![format!("Stored directory: {}", current_dir.clone())]], true, false);
    print!("=== Basic Command Substitution Complete ===\n");
    std::process::ExitCode::SUCCESS
}