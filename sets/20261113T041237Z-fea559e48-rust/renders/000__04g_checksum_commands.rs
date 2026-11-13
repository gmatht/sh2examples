use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_d: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", "=== Checksum Commands ===".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_d.with(|v| *v.borrow_mut() = { let (__c, __r) = { let __sc = std::process::Command::new("mktemp").arg("-d").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c });
    if !{ let __d = __SHV_d.with(|v| v.borrow().clone()); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() } {
        std::process::exit(1);
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("test_checksum.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "test content\n".as_bytes());
    }
    let sha256_result = { let (__c, __r) = { let __sc = std::process::Command::new("sha256sum").arg("test_checksum.txt").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c };
    println!("{}", format!("SHA256 result: {}", sha256_result.clone()));
    let sha512_result = { let (__c, __r) = { let __sc = std::process::Command::new("sha512sum").arg("test_checksum.txt").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c };
    println!("{}", format!("SHA512 result: {}", sha512_result.clone()));
    let strings_result = {
    let __cap0_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap0_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap0_in = __SH_STDIN.lock().unwrap().take();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("strings").arg("target/debug/debashc.exe").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("head").arg("-3").stdin(__x).stdout(std::process::Stdio::piped()).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &[]); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    *__SH_STDIN.lock().unwrap() = __cap0_in;
    let __cap0_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap0_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap0_outf);
    let mut __s = String::from_utf8_lossy(&__cap0_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    println!("{}", "Strings result:".to_string());
    println!("{}", strings_result.clone());
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg("test_checksum.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", "=== Checksum Commands Complete ===".to_string());
    let _ = { let __d = "/".to_string(); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() };
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-r").arg("-f").arg(__SHV_d.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}