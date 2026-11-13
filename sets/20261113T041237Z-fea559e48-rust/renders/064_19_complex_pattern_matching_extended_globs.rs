use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_d: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_d.with(|v| *v.borrow_mut() = { let (__c, __r) = { let __sc = std::process::Command::new("mktemp").arg("-d").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c });
    if !{ let __d = __SHV_d.with(|v| v.borrow().clone()); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() } {
        std::process::exit(1);
    }
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("touch").arg("a.txt").arg("b.log").arg("c.dat").arg("x.yz").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    for file in ["\x01SH2GLOB\x01*.txt", "\x01SH2GLOB\x01*.log", "\x01SH2GLOB\x01*.dat"] {
        let __sh_case1 = file;
        if __sh_case1.ends_with(".txt") || __sh_case1.ends_with(".log") {
            println!("{}", format!("Text file: {}", file));
        } else if (__sh_case1.ends_with(".dat")) {
            println!("{}", format!("Data file: {}", file));
        } else if true {
            println!("{}", format!("Other file: {}", file));
        } else {
        }
    }
    let _ = { let __d = "/".to_string(); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() };
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-r").arg("-f").arg(__SHV_d.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}