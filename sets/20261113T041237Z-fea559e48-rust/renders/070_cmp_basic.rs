use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);

thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp0: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp1: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp2: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp3: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("/tmp/cmp_a.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "abcdefghij\n".as_bytes());
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("/tmp/cmp_same.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "abcdefghij\n".as_bytes());
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("/tmp/cmp_diff.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "abcdeZghij\n".as_bytes());
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("/tmp/cmp_diff2.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "xyzdefghij\n".as_bytes());
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("/tmp/cmp_short.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "abc\n".as_bytes());
    }
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("touch").arg("/tmp/cmp_empty.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_same.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_diff.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_empty.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("/tmp/cmp_empty.txt").arg("/tmp/cmp_empty.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-s").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_diff.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("-s exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-s").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_same.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("-s same exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-l").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_diff.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("-l exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-b").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_diff.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("-b exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-n").arg("5").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_diff.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("-n 5 exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-n").arg("10").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_diff.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("-n 10 exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-n").arg("10").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_short.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("-n 10 short exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-i").arg("6").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_diff.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("-i 6 exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-i").arg("3").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_diff.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("-i 3 exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-i").arg("0:6").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_diff.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("-i 0:6 exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-i").arg("5:0").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_diff2.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("-i 5:0 exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SHV___ps_tmp0.with(|v| *v.borrow_mut() = {
    let (__c, __r) = { let __sc = std::process::Command::new("mktemp").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
});
    __SH_RC.store(0, Ordering::SeqCst);
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create(&__SHV___ps_tmp0.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "a\n".as_bytes());
    }
    __SHV___ps_tmp1.with(|v| *v.borrow_mut() = {
    let __cap0_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap0_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap0_in = __SH_STDIN.lock().unwrap().take();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mktemp").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap0_in;
    let __cap0_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap0_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap0_outf);
    let mut __s = String::from_utf8_lossy(&__cap0_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    __SH_RC.store(0, Ordering::SeqCst);
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create(&__SHV___ps_tmp1.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "a\n".as_bytes());
    }
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-s").arg(__SHV___ps_tmp0.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp1.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp0.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp1.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
    println!("{}", format!("aa -s exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SHV___ps_tmp2.with(|v| *v.borrow_mut() = {
    let (__c, __r) = { let __sc = std::process::Command::new("mktemp").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
});
    __SH_RC.store(0, Ordering::SeqCst);
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create(&__SHV___ps_tmp2.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "b\n".as_bytes());
    }
    __SHV___ps_tmp3.with(|v| *v.borrow_mut() = {
    let (__c, __r) = { let __sc = std::process::Command::new("mktemp").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
});
    __SH_RC.store(0, Ordering::SeqCst);
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create(&__SHV___ps_tmp3.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "c\n".as_bytes());
    }
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cmp").arg("-s").arg(__SHV___ps_tmp2.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp3.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp2.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp3.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
    println!("{}", format!("bc -s exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg("/tmp/cmp_a.txt").arg("/tmp/cmp_same.txt").arg("/tmp/cmp_diff.txt").arg("/tmp/cmp_diff2.txt").arg("/tmp/cmp_short.txt").arg("/tmp/cmp_empty.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}