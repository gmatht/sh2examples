use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_arr: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_k: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_map: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_x: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_print_words(ws: &[Vec<String>], nl: bool, _esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    if nl { s.push('\n'); }
    __sh_sink(s.as_bytes());
}

fn __sh_spawn_argv(argv: &[&str], input: Option<&[u8]>) -> i32 {
    if argv.is_empty() { return 0; }
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
    let mut c = std::process::Command::new(argv[0]);
    c.args(&argv[1..]);
    if !want_out {
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
    if want_out { c.stdout(std::process::Stdio::piped()); }
    if let (None, Some(p)) = (&input, &stdin_path) {
        if let Ok(f) = std::fs::File::open(p) {
            c.stdin(std::process::Stdio::from(f));
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

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    print!("== Indexed arrays ==\n");
    __SHV_arr.with(|v| *v.borrow_mut() = vec!["one".to_string(), "two".to_string(), "three".to_string()]);
    __SH_RC.store(0, Ordering::Relaxed);
    println!("{}", { let __sh_idx = 1; let __sh_num = __SHV_arr.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_arr.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } });
    println!("{}", __SHV_arr.with(|v| v.borrow().len() as i64));
    __SH_RC.store(0, Ordering::Relaxed);
    let __sh_items0 = { let __sh_value = __SHV_arr.with(|v| v.borrow().clone()); let __sh_offset = if 0 < 0 { (__sh_value.len() as i64 + 0).max(0) } else { 0 } as usize; let __sh_offset = __sh_offset.min(__sh_value.len()); let __sh_len = if -1 < 0 { __sh_value.len() as i64 - __sh_offset as i64 } else { -1 };                          let __sh_start = __sh_offset.max(0) as usize; let __sh_stop = ((__sh_start as i64 + __sh_len).max(__sh_start as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_start);                          __sh_value[__sh_start..__sh_stop].to_vec() };
    let mut __sh_index1: usize = 0;
    loop {
    if !(__sh_index1 < __sh_items0.len()) { break; }
        __SHV_x.with(|v| *v.borrow_mut() = __sh_items0[__sh_index1].clone());
        print!("{} ", __SHV_x.with(|v| v.borrow().clone()));
    __sh_index1 += 1;
    }
    println!();
    print!("== Associative arrays ==\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_map.with(|v| { v.borrow_mut().insert("foo".to_string(), "bar".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_map.with(|v| { v.borrow_mut().insert("answer".to_string(), "42".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_map.with(|v| { v.borrow_mut().insert("two".to_string(), "1 + 1".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    println!("{}", __SHV_map.with(|v| v.borrow().get("foo").cloned().unwrap_or_default()));
    println!("{}", __SHV_map.with(|v| v.borrow().get("answer").cloned().unwrap_or_default()));
    let mut __sh_pbuf2: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout3 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __sh_items4 = __SHV_map.with(|v| v.borrow().keys().cloned().collect::<Vec<String>>());
    let mut __sh_index5: usize = 0;
    loop {
    if !(__sh_index5 < __sh_items4.len()) { break; }
        __SHV_k.with(|v| *v.borrow_mut() = __sh_items4[__sh_index5].clone());
        __sh_print_words(&[vec![format!("{} => {}", __SHV_k.with(|v| v.borrow().clone()), __SHV_map.with(|v| v.borrow().get(&__SHV_k.with(|v| v.borrow().clone())).cloned().unwrap_or_default()))]], true, false);
        __SH_RC.store(0, Ordering::Relaxed);
    __sh_index5 += 1;
    }
    __sh_pbuf2 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout3);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn_argv(&["sort"], Some(&__sh_pbuf2)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}