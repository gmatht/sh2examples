use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV___sh2_tc0_: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_array: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_result: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn __sh_print_words(ws: &[Vec<String>], nl: bool, _esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    if nl { s.push('\n'); }
    __sh_sink(s.as_bytes());
}

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
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

fn __sh_regex(text: &str, pat: &str, flags: &str) -> Option<bool> {
    let pat = if flags.contains('i') { format!("(?i){}", pat) } else { pat.to_string() };
    match regex::Regex::new(&pat) {
        Ok(re) => Some(re.is_match(text)),
        Err(_) => None,
    }
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
    let mut __sh_pbuf0: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout1 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __sh_sv2 = __SHV___sh2_tc0_.with(|v| v.borrow().clone());
    let __sh_sv3 = __SHV_array.with(|v| v.borrow().clone());
    let __sh_sv4 = __SHV_i.with(|v| v.borrow().clone());
    let __sh_sv5 = __SHV_result.with(|v| v.borrow().clone());
    __SHV___sh2_tc0_.with(|v| *v.borrow_mut() = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf6: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout7 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_print_words(&[vec![std::env::var("var").unwrap_or_default()]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf6 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout7);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn("'tr' '[:upper:]' '[:lower:]'", Some(&__sh_pbuf6)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
});
    if { let __sh_mapped = __sh_regex(&__SHV___sh2_tc0_.with(|v| v.borrow().clone()).to_string(), &"^[a-z]+$".to_string().to_string(), ""); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } } {
        let __sh_arith_value8 = { let __sh_value = 0; __SHV_i.with(|v| *v.borrow_mut() = __sh_value.to_string()); __sh_value };
        let __sh_num9 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value8 };
        __SH_RC.store(if __sh_num9 != 0 { 0 } else { 1 }, Ordering::Relaxed);
        let mut __sh_while_ran10 = false;
        let mut __sh_while_last11 = 0;
        while __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) < __SHV_array.with(|v| v.borrow().len() as i64) {
            __sh_while_ran10 = true;
            if __SHV_array.with(|v| v.borrow().get(__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) > std::env::var("threshold").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) && std::path::Path::new(&std::env::var("files[i]").unwrap_or_default()).is_file() {
                { let __sh_piece = (__SHV_result.with(|v| v.borrow().get(__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) + __sh_capture_rc(&"(wc -l < \"${files[i]}\")").0.trim().parse::<i64>().unwrap_or(0)).to_string(); __SHV_result.with(|v| { let mut b = v.borrow_mut(); let i = __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) as usize; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __sh_piece; }); }
                __SH_RC.store(0, Ordering::Relaxed);
            } else {
                __SH_RC.store(0, Ordering::Relaxed);
            }
            let __sh_arith_value12 = { let __sh_out = __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)); __SHV_i.with(|v| *v.borrow_mut() = (__SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) +1).to_string()); __sh_out };
            let __sh_num13 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value12 };
            __SH_RC.store(if __sh_num13 != 0 { 0 } else { 1 }, Ordering::Relaxed);
            __sh_while_last11 = __SH_RC.load(Ordering::Relaxed);
        }
        if !__sh_while_ran10 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last11, Ordering::Relaxed); }
    } else {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    __SHV___sh2_tc0_.with(|v| *v.borrow_mut() = __sh_sv2);
    __SHV_array.with(|v| *v.borrow_mut() = __sh_sv3);
    __SHV_i.with(|v| *v.borrow_mut() = __sh_sv4);
    __SHV_result.with(|v| *v.borrow_mut() = __sh_sv5);
    __sh_pbuf0 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout1);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'sort' '-n'", format!("{} > {}", "'tail' '-n' '5'".to_string(), __sh_q("final_result.txt"))), Some(&__sh_pbuf0)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}