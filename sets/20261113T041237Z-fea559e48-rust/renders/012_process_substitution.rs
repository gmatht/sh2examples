use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp0: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp1: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp2: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp3: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp4: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp5: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV___ps_tmp6: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_lines: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

fn __sh_readline() -> (String, bool) {
    let mut buf: Vec<u8> = Vec::new();
    let mut one = [0u8; 1];
    let mut any = false;
    let src: Box<dyn std::io::Read + Send> = match __SH_STDIN.lock().unwrap().take() {
        Some(r) => r,
        None => Box::new(std::io::stdin()),
    };
    let mut r = src;
    loop {
        match r.read(&mut one) {
            Ok(0) => break,
            Ok(_) => {
                if one[0] == b'\n' { any = true; break; }
                buf.push(one[0]);
                any = true;
            }
            Err(_) => break,
        }
    }
    *__SH_STDIN.lock().unwrap() = Some(r);
    (String::from_utf8_lossy(&buf).to_string(), any)
}

fn __sh_grepmatches(text: &str, pat: &str, flags: &str) -> (String, i32) {
    let mut opts = String::from("-o");
    if flags.contains('i') { opts.push('i'); }
    if flags.contains('E') { opts.push('E'); }
    let cmd = format!("printf '%s' \"$1\" | grep {opts} \"$2\"");
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(&cmd).arg("sh").arg(text).arg(pat);
    c.stdout(std::process::Stdio::piped());
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let mut s = String::from_utf8_lossy(&out).to_string();
    while s.ends_with('\n') { s.pop(); }
    (s, rc)
}

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", "== Here-string with grep -o ==".to_string());
    let (__m, __r) = __sh_grepmatches("some pattern here", "pattern", "");
    if !__m.is_empty() { println!("{}", __m); }
    println!("{}", "== Process substitution with comm ==".to_string());
    __SHV___ps_tmp0.with(|v| *v.borrow_mut() = {
    let (__c, __r) = { let __sc = std::process::Command::new("mktemp").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
});
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create(&__SHV___ps_tmp0.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "a\nb\n".as_bytes());
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
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create(&__SHV___ps_tmp1.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "b\nc\n".as_bytes());
    }
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("comm").arg("-12").arg(__SHV___ps_tmp0.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp1.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp0.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp1.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
    println!("{}", "== readarray/mapfile ==".to_string());
    __SHV___ps_tmp2.with(|v| *v.borrow_mut() = {
    let (__c, __r) = { let __sc = std::process::Command::new("mktemp").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
});
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create(&__SHV___ps_tmp2.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "x\ny\n".as_bytes());
    }
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open(&__SHV___ps_tmp2.with(|v| v.borrow().clone())).unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from(&__SHV___ps_tmp2.with(|v| v.borrow().clone())));
        { let mut __lines: Vec<String> = Vec::new(); loop { let (__ln, __any) = __sh_readline(); if !__any { break; } __lines.push(__ln); } __SHV_lines.with(|v| *v.borrow_mut() = __lines); }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp2.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
    print!("{}", format!("{} ", __SHV_lines.with(|v| v.borrow().clone()).join(" ")));
    println!("{}", "".to_string());
    println!("{}", "== More process substitution examples ==".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    if !{
{
    __SHV___ps_tmp3.with(|v| *v.borrow_mut() = {
    let __cap1_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap1_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap1_in = __SH_STDIN.lock().unwrap().take();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mktemp").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap1_in;
    let __cap1_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap1_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap1_outf);
    let mut __s = String::from_utf8_lossy(&__cap1_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    __SH_RC.store(0, Ordering::SeqCst);
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp3.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        let mut __sh_pbuf0: Vec<u8> = Vec::new();
        __SH_PIPESTATUS.lock().unwrap().clear();
        {
    let __sh_oldout1 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"a\nc\nb\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf0 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout1);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
        __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("sort").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf0); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
        __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    __SHV___ps_tmp4.with(|v| *v.borrow_mut() = {
    let __cap2_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap2_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap2_in = __SH_STDIN.lock().unwrap().take();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mktemp").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap2_in;
    let __cap2_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap2_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap2_outf);
    let mut __s = String::from_utf8_lossy(&__cap2_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    __SH_RC.store(0, Ordering::SeqCst);
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create(&__SHV___ps_tmp4.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        let mut __sh_pbuf2: Vec<u8> = Vec::new();
        __SH_PIPESTATUS.lock().unwrap().clear();
        {
    let __sh_oldout3 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"a\nb\nd\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf2 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout3);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
        __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("sort").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf2); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
        __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("diff").arg(__SHV___ps_tmp3.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp4.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    if __SH_RC.load(Ordering::SeqCst) == 0 {
        let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp3.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp4.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
    }
}
__SH_RC.load(Ordering::SeqCst) == 0
} {
        print!("{}", "Files differ\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    __SHV___ps_tmp5.with(|v| *v.borrow_mut() = {
    let __cap3_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap3_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap3_in = __SH_STDIN.lock().unwrap().take();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mktemp").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap3_in;
    let __cap3_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap3_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap3_outf);
    let mut __s = String::from_utf8_lossy(&__cap3_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    __SH_RC.store(0, Ordering::SeqCst);
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create(&__SHV___ps_tmp5.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "name1\nname2\n".as_bytes());
    }
    __SHV___ps_tmp6.with(|v| *v.borrow_mut() = {
    let (__c, __r) = { let __sc = std::process::Command::new("mktemp").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
});
    __SH_RC.store(0, Ordering::SeqCst);
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create(&__SHV___ps_tmp6.with(|v| v.borrow().clone())).unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "value1\nvalue2\n".as_bytes());
    }
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("paste").arg(__SHV___ps_tmp5.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp6.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    let _ = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(__SHV___ps_tmp5.with(|v| v.borrow().clone())).arg(__SHV___ps_tmp6.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } };
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}