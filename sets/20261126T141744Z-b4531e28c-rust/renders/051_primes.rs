use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_n: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_primes: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_z: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

fn __sh_print_words(ws: &[Vec<String>], nl: bool, _esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    if nl { s.push('\n'); }
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

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
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
    let mut y = String::new();
    let __sh_arith_value4 = ({ let __sh_value = y.trim().parse::<i64>().unwrap_or(0) + 2; y = ((y.trim().parse::<i64>().unwrap_or(0) + 2)).to_string(); __sh_value }).to_string();
    { let __sh_value = __sh_arith_value4; let __sh_num = __sh_value.trim().parse::<i64>().unwrap_or(0); __SH_RC.store(if __sh_num != 0 { 0 } else { 1 }, Ordering::Relaxed); __sh_value };
    __SHV_z.with(|v| { let mut b = v.borrow_mut(); for __sh_word in vec!["a".to_string(), "b".to_string()] { b.push(__sh_word); } });
    __SH_RC.store(0, Ordering::Relaxed);
    let __sh_arith_value5 = ({ let __sh_value = __SHV_z.with(|v| v.borrow().get(0 as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) + __SHV_primes.with(|v| v.borrow().clone()).join(" ").trim().parse::<i64>().unwrap_or(0); { let __sh_piece = ((__SHV_z.with(|v| v.borrow().get(0 as usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) + __SHV_primes.with(|v| v.borrow().clone()).join(" ").trim().parse::<i64>().unwrap_or(0))).to_string(); __SHV_z.with(|v| { let mut b = v.borrow_mut(); let i = 0 as usize; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __sh_piece; }); } __sh_value }).to_string();
    { let __sh_value = __sh_arith_value5; let __sh_num = __sh_value.trim().parse::<i64>().unwrap_or(0); __SH_RC.store(if __sh_num != 0 { 0 } else { 1 }, Ordering::Relaxed); __sh_value };
    print!("=== Prime Number Generator (first 1000 primes) ===\n");
    print!("Finding first 100 prime numbers...\n");
    print!("This may take a while...\n");
    __SHV_primes.with(|v| *v.borrow_mut() = vec!["2".to_string()]);
    let mut count = 1;
    let mut candidate = 3;
    __SH_RC.store(0, Ordering::Relaxed);
    while count < 100 {
        if { let __sh_operand: Vec<String> = { let __sh_split_s: String = candidate.to_string(); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }; is_prime(__sh_operand.get(0).cloned().unwrap_or_default()); __SH_RC.load(Ordering::Relaxed) == 0 } {
            __SHV_primes.with(|v| { let mut b = v.borrow_mut(); for __sh_word in { let __sh_split_s: String = candidate.to_string(); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() } { b.push(__sh_word); } });
            count = count + 1;
            if (count % 10) == 0 {
                __sh_print_words(&[vec![format!("Found {} primes so far...", count)]], true, false);
            }
        }
        candidate = candidate + 2;
    }
    println!();
    print!("First 1000 prime numbers found!\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("Count: {}", __SHV_primes.with(|v| v.borrow().len() as i64).to_string());
    println!("First 10: {}", { let __sh_value = __SHV_primes.with(|v| v.borrow().clone()); let __sh_offset = if 0 < 0 { (__sh_value.len() as i64 + 0).max(0) } else { 0 } as usize; let __sh_offset = __sh_offset.min(__sh_value.len()); let __sh_len = if 10 < 0 { __sh_value.len() as i64 - __sh_offset as i64 } else { 10 };                          let __sh_start = __sh_offset.max(0) as usize; let __sh_stop = ((__sh_start as i64 + __sh_len).max(__sh_start as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_start);                          __sh_value[__sh_start..__sh_stop].join(" ") });
    println!("Last 10: {}", { let __sh_value = __SHV_primes.with(|v| v.borrow().clone()); let __sh_offset = if -10 < 0 { (__sh_value.len() as i64 + -10).max(0) } else { -10 } as usize; let __sh_offset = __sh_offset.min(__sh_value.len()); let __sh_len = if i64::MIN < 0 { __sh_value.len() as i64 - __sh_offset as i64 } else { i64::MIN };                          let __sh_start = __sh_offset.max(0) as usize; let __sh_stop = ((__sh_start as i64 + __sh_len).max(__sh_start as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_start);                          __sh_value[__sh_start..__sh_stop].join(" ") });
    print!("Prime number generation complete!\n");
    __SH_RC.store(0, Ordering::Relaxed);
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn is_prime(p0: String) {
    let __sh_loc_n: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
    __SHV_n.with(|v| *v.borrow_mut() = format!("{}", p0.clone()));
    __SH_RC.store(0, Ordering::Relaxed);
    if __SHV_n.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) < 2 {
        let __sh_value = "1".to_string(); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        match __sh_loc_n.take() { Some(__sh_loc) => { __SHV_n.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
        return;
    } else {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    if __SHV_n.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) == 2 {
        let __sh_value = "0".to_string(); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        match __sh_loc_n.take() { Some(__sh_loc) => { __SHV_n.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
        return;
    } else {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    if (__SHV_n.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) % 2i64) == 0 {
        let __sh_value = "1".to_string(); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        match __sh_loc_n.take() { Some(__sh_loc) => { __SHV_n.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
        return;
    } else {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    let sqrt_n = { let __sh_split_s: String = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf8: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout9 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_print_words(&[vec![format!("sqrt({})", __SHV_n.with(|v| v.borrow().clone()))]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf8 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout9);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn("'bc'", Some(&__sh_pbuf8)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
}; __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }.join(" ");
    let mut i = 3;
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_while_ran10 = false;
    let mut __sh_while_last11 = 0;
    while i <= sqrt_n.trim().parse::<i64>().unwrap_or(0) {
        __sh_while_ran10 = true;
        if (__SHV_n.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) % i).to_string().trim().parse::<i64>().unwrap_or(0) == 0 {
            let __sh_value = "1".to_string(); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
            match __sh_loc_n.take() { Some(__sh_loc) => { __SHV_n.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
            return;
        } else {
            __SH_RC.store(0, Ordering::Relaxed);
        }
        i = i + 2;
        __SH_RC.store(0, Ordering::Relaxed);
        __sh_while_last11 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran10 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last11, Ordering::Relaxed); }
    let __sh_value = "0".to_string(); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
    match __sh_loc_n.take() { Some(__sh_loc) => { __SHV_n.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
    return;
    match __sh_loc_n.take() { Some(__sh_loc) => { __SHV_n.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
}