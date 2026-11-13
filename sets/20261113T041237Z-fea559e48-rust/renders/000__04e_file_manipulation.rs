use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_d: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", "=== File Manipulation Commands ===".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_d.with(|v| *v.borrow_mut() = { let (__c, __r) = { let __sc = std::process::Command::new("mktemp").arg("-d").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c });
    if !{ let __d = __SHV_d.with(|v| v.borrow().clone()); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() } {
        std::process::exit(1);
    }
    {
    __SH_RC.store(0, Ordering::SeqCst);
    let _ = std::io::Write::write_all(&mut std::fs::File::create("test_file.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "test content\n".as_bytes());
    }
    let cp_result = {
    let __cap0_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap0_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap0_in = __SH_STDIN.lock().unwrap().take();
    if { let __r = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("cp").arg("test_file.txt").arg("test_file_copy.txt").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }; __SH_RC.store(__r, Ordering::SeqCst); __r == 0 } {
        { let __t: &str = {"Copy successful\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
        __SH_RC.store(0, Ordering::SeqCst);
    }
    *__SH_STDIN.lock().unwrap() = __cap0_in;
    let __cap0_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap0_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap0_outf);
    let mut __s = String::from_utf8_lossy(&__cap0_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    println!("{}", format!("Copy result: {}", cp_result.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    if !{
__SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::fs::File::create("/dev/null") { Ok(__rf) => match std::process::Command::new("ls").arg("test_file.txt").arg("test_file_copy.txt").arg("test_file_moved.txt").stderr(__rf).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } }, Err(_) => 1 } }, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
} {
        print!("{}", "No test files found\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    let mv_result = {
    let __cap1_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap1_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap1_in = __SH_STDIN.lock().unwrap().take();
    if { let __r = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mv").arg("test_file_copy.txt").arg("test_file_moved.txt").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }; __SH_RC.store(__r, Ordering::SeqCst); __r == 0 } {
        { let __t: &str = {"Move successful\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
        __SH_RC.store(0, Ordering::SeqCst);
    }
    *__SH_STDIN.lock().unwrap() = __cap1_in;
    let __cap1_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap1_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap1_outf);
    let mut __s = String::from_utf8_lossy(&__cap1_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    println!("{}", format!("Move result: {}", mv_result.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    if !{
__SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::fs::File::create("/dev/null") { Ok(__rf) => match std::process::Command::new("ls").arg("test_file.txt").arg("test_file_copy.txt").arg("test_file_moved.txt").stderr(__rf).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } }, Err(_) => 1 } }, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
} {
        print!("{}", "No test files found\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    let rm_result = {
    let __cap2_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap2_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap2_in = __SH_STDIN.lock().unwrap().take();
    if { let __r = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("test_file.txt").arg("test_file_moved.txt").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }; __SH_RC.store(__r, Ordering::SeqCst); __r == 0 } {
        { let __t: &str = {"Remove successful\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
        __SH_RC.store(0, Ordering::SeqCst);
    }
    *__SH_STDIN.lock().unwrap() = __cap2_in;
    let __cap2_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap2_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap2_outf);
    let mut __s = String::from_utf8_lossy(&__cap2_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    println!("{}", format!("Remove result: {}", rm_result.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    if !{
__SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::fs::File::create("/dev/null") { Ok(__rf) => match std::process::Command::new("ls").arg("test_file.txt").arg("test_file_copy.txt").arg("test_file_moved.txt").stderr(__rf).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } }, Err(_) => 1 } }, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
} {
        print!("{}", "No test files found\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    let mkdir_result = {
    let __cap3_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap3_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap3_in = __SH_STDIN.lock().unwrap().take();
    if { let __r = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mkdir").arg("test_dir").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }; __SH_RC.store(__r, Ordering::SeqCst); __r == 0 } {
        { let __t: &str = {"Directory created\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
        __SH_RC.store(0, Ordering::SeqCst);
    }
    *__SH_STDIN.lock().unwrap() = __cap3_in;
    let __cap3_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap3_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap3_outf);
    let mut __s = String::from_utf8_lossy(&__cap3_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    println!("{}", format!("Mkdir result: {}", mkdir_result.clone()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("touch").arg("test_dir/file").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    if !{
__SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::fs::File::create("/dev/null") { Ok(__rf) => match std::process::Command::new("ls").arg("test_dir").stderr(__rf).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } }, Err(_) => 1 } }, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
} {
        print!("{}", "Directory not found\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    let touch_result = {
    let __cap4_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap4_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap4_in = __SH_STDIN.lock().unwrap().take();
    if { let __r = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("touch").arg("test_file.txt").stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }; __SH_RC.store(__r, Ordering::SeqCst); __r == 0 } {
        { let __t: &str = {"File touched\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
        __SH_RC.store(0, Ordering::SeqCst);
    }
    *__SH_STDIN.lock().unwrap() = __cap4_in;
    let __cap4_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap4_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap4_outf);
    let mut __s = String::from_utf8_lossy(&__cap4_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    println!("{}", format!("Touch result: {}", touch_result.clone()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg("test_file.txt").arg("test_file_copy.txt").arg("test_file_moved.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    if !{
__SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::fs::File::create("/dev/null") { Ok(__rf) => match std::process::Command::new("rm").arg("-r").arg("-f").arg("test_dir").stderr(__rf).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } }, Err(_) => 1 } }, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
} {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    println!("{}", "=== File Manipulation Commands Complete ===".to_string());
    let _ = { let __d = "/".to_string(); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() };
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-r").arg("-f").arg(__SHV_d.with(|v| v.borrow().clone())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}