use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_d: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_d.with(|v| *v.borrow_mut() = { let (__c, __r) = { let __sc = std::process::Command::new("mktemp").arg("-d").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c });
    if !{ let __d = __SHV_d.with(|v| v.borrow().clone()); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() } {
        std::process::exit(1);
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("alpha").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "alpha\n".as_bytes());
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("beta").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "beta\n".as_bytes());
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("gamma").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "gamma\n".as_bytes());
    }
    println!("{}", "File list:".to_string());
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("ls").arg("-1").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("head").arg("-3").stdin(__x).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &[]); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    println!("{}", "---".to_string());
    println!("{}", "Count:".to_string());
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("ls").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("wc").arg("-l").stdin(__x).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &[]); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    println!("{}", "done".to_string());
    let _ = { let __d = "/".to_string(); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() };
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-r").arg("-f").arg(__SHV_d.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}