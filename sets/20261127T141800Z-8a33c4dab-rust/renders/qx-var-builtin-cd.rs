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

fn __sh_dirname_args(argv: &[String]) -> Option<String> {
    if argv.is_empty() {
        eprintln!("dirname: missing operand\nTry 'dirname --help' for more information.");
        return None;
    }
    let mut out: Vec<String> = Vec::new();
    for p in argv {
        if p.is_empty() {
            out.push(".".to_string());
            continue;
        }
        let t = p.trim_end_matches('/');
        if t.is_empty() {
            out.push("/".to_string());
            continue;
        }
        out.push(match t.rfind('/') {
            None => ".".to_string(),
            Some(i) => {
                let d = t[..i].trim_end_matches('/');
                if d.is_empty() { "/".to_string() } else { d.to_string() }
            }
        });
    }
    Some(out.join("\n"))
}

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

fn main() -> std::process::ExitCode {
    let script_dir = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    { let __sh_buf = { let __sh_dir = match __sh_dirname_args(&vec![std::env::args().next().unwrap_or_default()]) { Some(__sh_ch) => { __SH_RC.store(0, Ordering::Relaxed); __sh_ch } None => { __SH_RC.store(1, Ordering::Relaxed); String::new() } }; let __sh_res = std::env::set_current_dir(&__sh_dir); if __sh_res.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __sh_res.is_ok() { 0 } else { 1 }, Ordering::Relaxed); __sh_res.is_ok() }; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if __sh_buf {
        __sh_print_words(&[vec![std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()]], true, false);
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
    println!("{}", script_dir.clone());
    std::process::ExitCode::SUCCESS
}