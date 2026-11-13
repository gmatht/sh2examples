use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_d: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", "=== File and Directory Operations ===".to_string());
    __SHV_d.with(|v| *v.borrow_mut() = { let (__c, __r) = { let __sc = std::process::Command::new("mktemp").arg("-d").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c });
    if !{ let __d = __SHV_d.with(|v| v.borrow().clone()); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() } {
        std::process::exit(1);
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("a.sh").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "a\n".as_bytes());
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("b.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "b\n".as_bytes());
    }
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mkdir").arg("sub").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("sub/c.sh").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "c\n".as_bytes());
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create(".hidden").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "x\n".as_bytes());
    }
    let file_list = { let (__c, __r) = { let __sc = std::process::Command::new("ls").arg("-A").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c };
    println!("{}", "File listing:".to_string());
    println!("{}", file_list.clone());
    let found_files = { let (__c, __r) = { let __sc = std::process::Command::new("find").arg(".").arg("-name").arg("*.sh").arg("-type").arg("f").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c };
    println!("{}", "Found shell scripts:".to_string());
    println!("{}", found_files.clone());
    let _ = { let __d = "/".to_string(); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() };
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-r").arg("-f").arg(__SHV_d.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", "=== File and Directory Operations Complete ===".to_string());
}