use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp0: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    __SHV___ps_tmp0.with(|v| *v.borrow_mut() = {
    let (__c, __r) = { let __sc = std::process::Command::new("mktemp").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
});
    __SH_RC.store(0, Ordering::SeqCst);
    { let __p = __SHV___ps_tmp0.with(|v| v.borrow().clone()); let _ = std::fs::remove_file(&__p); let _ = std::process::Command::new("mkfifo").arg(&__p).status(); }
    let __ps_tmp_loc = __SHV___ps_tmp0.with(|v| v.borrow().clone());
    std::thread::spawn(move || { 
    let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__ps_tmp_loc).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
    while true {
        { let __w: &[Vec<String>] = {&[vec![".".to_string()]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
        std::thread::sleep(std::time::Duration::from_secs_f64(("1".to_string()).trim().parse::<f64>().unwrap_or(0.0).max(0.0)));
    }
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
 });
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("head").arg(__SHV___ps_tmp0.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp0.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}