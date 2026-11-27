use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_array: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

fn __sh_print_words(ws: &[Vec<String>], nl: bool, _esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    if nl { s.push('\n'); }
    __sh_sink(s.as_bytes());
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
    println!("{}", { let __sh_value = std::env::var("var").unwrap_or_default(); if __sh_value.is_empty() { format!("{}", { let __sh_value = std::env::var("default").unwrap_or_default(); if __sh_value.is_empty() { format!("{}", { let __sh_value = std::env::var("fallback").unwrap_or_default(); if __sh_value.is_empty() { "computed".to_string() } else { __sh_value.to_string() } }) } else { __sh_value.to_string() } }) } else { __sh_value.to_string() } });
    println!("{}", { let __sh_idx = 0; let __sh_num = __SHV_array.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_array.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } });
    __SH_ARITH_WORD_FAIL.store(false, Ordering::Relaxed);
    { let __sh_wordlist = vec![{ __SH_ARITH_WORD_FAIL.store(true, Ordering::Relaxed); String::new() }]; let __sh_wordfail = __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed); if !__sh_wordfail { __sh_print_words(&[__sh_wordlist], true, false); } __SH_RC.store(if __sh_wordfail { 1 } else { 0 }, Ordering::Relaxed); }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}