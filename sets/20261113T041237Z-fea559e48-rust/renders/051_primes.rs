use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_n: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_primes: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_y: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_z: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

fn __sh_spawn(cmd: &str, input: Option<&[u8]>) -> i32 {
    // a shell-out inside a native stage inherits the stage's stdin
    // (`echo x | grep -f <(…)` — the grep reads the echo's buffer)
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
        // the child inherits fd 1 raw — flush Rust's buffered stdout
        // first so prints BEFORE the child hit the pipe first
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
    if want_out { c.stdout(std::process::Stdio::piped()); }
    if input.is_none() {
        if let Some(p) = &stdin_path {
            // the child must see the REAL device fd (`tty < /dev/pts/5`
            // needs isatty, which a byte pipe lacks)
            if let Ok(f) = std::fs::File::open(p) {
                c.stdin(std::process::Stdio::from(f));
            }
        }
    }
    if input.is_some() { c.stdin(std::process::Stdio::piped()); }
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return 1 };
    if let Some(data) = input {
        if let Some(mut si) = ch.stdin.take() { let _ = std::io::Write::write_all(&mut si, &data); }
    }
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    if !out.is_empty() {
        let mut used = false;
        __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &out); used = true; } });
        if !used {
            if let Some(b) = __SH_OUT.lock().unwrap().as_mut() {
                b.extend_from_slice(&out);
            } else {
                let _ = std::io::Write::write_all(&mut std::io::stdout(), &out);
            }
        }
    }
    rc
}

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    let __sh_arith_v4 = ({ let __v = (__SHV_y.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + 2); __SHV_y.with(|v| *v.borrow_mut() = ((__SHV_y.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + 2)).to_string()); __v }).to_string();
    let _ = { let __v = __sh_arith_v4; let __n = __v.trim().parse::<i64>().unwrap_or(0); __SH_RC.store(if __n != 0 { 0 } else { 1 }, Ordering::SeqCst); __v };
    __SHV_z.with(|v| { let mut b = v.borrow_mut(); for __w in vec!["a".to_string(), "b".to_string()] { b.push(__w); } });
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_arith_v5 = ({ let __v = (__SHV_z.with(|v| v.borrow().get(0usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) + __SHV_primes.with(|v| v.borrow().clone()).join(" ").trim().parse::<i64>().unwrap_or(0)); { let __val = ((__SHV_z.with(|v| v.borrow().get(0usize).cloned().unwrap_or_default()).trim().parse::<i64>().unwrap_or(0) + __SHV_primes.with(|v| v.borrow().clone()).join(" ").trim().parse::<i64>().unwrap_or(0))).to_string(); __SHV_z.with(|v| { let mut b = v.borrow_mut(); let i = 0; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __val; }); } __v }).to_string();
    let _ = { let __v = __sh_arith_v5; let __n = __v.trim().parse::<i64>().unwrap_or(0); __SH_RC.store(if __n != 0 { 0 } else { 1 }, Ordering::SeqCst); __v };
    println!("{}", "=== Prime Number Generator (first 1000 primes) ===".to_string());
    println!("{}", "Finding first 100 prime numbers...".to_string());
    println!("{}", "This may take a while...".to_string());
    __SHV_primes.with(|v| *v.borrow_mut() = vec!["2".to_string()]);
    let mut count = 1;
    let mut candidate = 3;
    __SH_RC.store(0, Ordering::SeqCst);
    while count < 100 {
        if { let __a: Vec<String> = [{ let __si = candidate.to_string(); let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }].concat(); is_prime(__a.get(0).cloned().unwrap_or_default()); __SH_RC.load(Ordering::SeqCst) == 0 } {
            __SHV_primes.with(|v| { let mut b = v.borrow_mut(); for __w in [{ let __si = candidate.to_string(); let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }].concat() { b.push(__w); } });
            count = count + 1;
            if { let __a = count; let __b = 10; if __b == 0 { __SH_ARITH_ERR.store(true, Ordering::SeqCst); 0 } else { __a % __b } } == 0 {
                println!("{}", format!("Found {} primes so far...", count));
            }
        }
        candidate = candidate + 2;
    }
    println!("{}", "".to_string());
    println!("{}", "First 1000 prime numbers found!".to_string());
    println!("{}", format!("Count: {}", __SHV_primes.with(|v| v.borrow().len()).to_string()));
    println!("{}", format!("First 10: {}", { let __v = __SHV_primes.with(|v| v.borrow().clone()); let __s = 0usize.min(__v.len()); let __e = (__s + 10usize).min(__v.len()); __v[__s..__e].join(" ") }));
    println!("{}", format!("Last 10: {}", { let __v = __SHV_primes.with(|v| v.borrow().clone()); let __s = __v.len().saturating_sub(10); __v[__s..].join(" ") }));
    println!("{}", "Prime number generation complete!".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
}

fn is_prime(p0: String) {
    __SHV_n.with(|v| *v.borrow_mut() = format!("{}", p0.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    if __SHV_n.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) < 2 {
        __SH_RC.store(1, Ordering::SeqCst); return;
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    if __SHV_n.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) == 2 {
        __SH_RC.store(0, Ordering::SeqCst); return;
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    if { let __a = __SHV_n.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); let __b = 2; if __b == 0 { __SH_ARITH_ERR.store(true, Ordering::SeqCst); 0 } else { __a % __b } } == 0 {
        __SH_RC.store(1, Ordering::SeqCst); return;
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    let sqrt_n = [{ let __si = {
    let __cap0_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap0_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap0_in = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf8: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout9 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __w: &[Vec<String>] = {&[vec![format!("sqrt({})", __SHV_n.with(|v| v.borrow().clone()))]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf8 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout9);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store(__sh_spawn("'bc'", Some(&__sh_pbuf8)), Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    *__SH_STDIN.lock().unwrap() = __cap0_in;
    let __cap0_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap0_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap0_outf);
    let mut __s = String::from_utf8_lossy(&__cap0_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
}; let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }].concat().join(" ");
    __SHV_i.with(|v| v.set(3));
    __SH_RC.store(0, Ordering::SeqCst);
    let mut __sh_while_ran10 = false;
    let mut __sh_while_last11 = 0;
    while __SHV_i.with(|v| v.get()) <= sqrt_n.clone().trim().parse::<i64>().unwrap_or(0) {
        __sh_while_ran10 = true;
        if (__SHV_n.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) % __SHV_i.with(|v| v.get())).to_string().trim().parse::<i64>().unwrap_or(0) == 0 {
            __SH_RC.store(1, Ordering::SeqCst); return;
        } else {
            __SH_RC.store(0, Ordering::SeqCst);
        }
        __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 2));
        __SH_RC.store(0, Ordering::SeqCst);
        __sh_while_last11 = __SH_RC.load(Ordering::SeqCst);
    }
    if !__sh_while_ran10 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last11, Ordering::SeqCst); }
    __SH_RC.store(0, Ordering::SeqCst); return;
}