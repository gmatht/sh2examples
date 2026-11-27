use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn __sh_print_words(ws: &[Vec<String>], nl: bool, _esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    if nl { s.push('\n'); }
    __sh_sink(s.as_bytes());
}

fn __sh_arg(i: usize) -> String {
    __SH_ARGV.lock().unwrap().get(i).cloned().unwrap_or_default()
}

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

fn main() -> std::process::ExitCode {
    __SH_ARITH_WORD_FAIL.store(false, Ordering::Relaxed);
    { let __sh_wordlist = vec![{ let __sh_arith_word = (((__sh_arg(0).trim().parse::<i64>().unwrap_or(0) * 100) + __sh_arg(1).trim().parse::<i64>().unwrap_or(0))).to_string(); if __sh_arg(0).trim().is_empty() || __sh_arg(1).trim().is_empty() { __SH_ARITH_WORD_FAIL.store(true, Ordering::Relaxed); } __sh_arith_word }]; let __sh_wordfail = __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed); if !__sh_wordfail { __sh_print_words(&[__sh_wordlist], true, false); } __SH_RC.store(if __sh_wordfail { 1 } else { 0 }, Ordering::Relaxed); }
    print!("n=[{}]\n", std::env::var("n").unwrap_or_default());
    std::process::ExitCode::SUCCESS
}