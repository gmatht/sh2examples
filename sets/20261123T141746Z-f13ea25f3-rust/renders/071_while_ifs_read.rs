use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_line: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    let _ = std::fs::write("/tmp/while_test.txt", "a\n");
    { let mut __sh_field = std::fs::OpenOptions::new().append(true).create(true).open("/tmp/while_test.txt").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f }); use std::io::Write; let _ = __sh_field.write_all("b\n".to_string().as_bytes()); }
    __SH_RC.store(0, Ordering::Relaxed);
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/tmp/while_test.txt").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/tmp/while_test.txt"));
        while {
let (__sh_line, __sh_gotline) = { let mut __sh_buf: Vec<u8> = Vec::new(); let mut __sh_one = [0u8; 1]; let mut __sh_any = false; let mut __sh_src: Box<dyn std::io::Read + Send> = match __SH_STDIN.lock().unwrap().take() { Some(__sh_r) => __sh_r, None => Box::new(std::io::stdin()) }; loop { match std::io::Read::read(&mut __sh_src, &mut __sh_one) { Ok(0) => break, Ok(_) => { __sh_any = true; if __sh_one[0] == b'\n' { break; } __sh_buf.push(__sh_one[0]); }, Err(_) => break } } *__SH_STDIN.lock().unwrap() = Some(__sh_src); (String::from_utf8_lossy(&__sh_buf).to_string(), __sh_any) };
let __sh_field: Vec<String> = vec![__sh_line.clone()];
__SHV_line.with(|v| *v.borrow_mut() = __sh_field.get(0).cloned().unwrap_or_default());
__SH_RC.store(if __sh_gotline { 0 } else { 1 }, Ordering::Relaxed);
__sh_gotline
} {
            println!("Line: {}", __SHV_line.with(|v| v.borrow().clone()));
        }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{}", __e) } };
        for __sh_path in &vec!["/tmp/while_test.txt".to_string()] {
            match std::fs::symlink_metadata(__sh_path) {
                Ok(__sh_mapped) => {
            if __sh_mapped.is_dir() {
                eprintln!("rm: cannot remove '{}': Is a directory", __sh_path);
                __sh_status = 1;
                continue;
            }
            let __sh_res = std::fs::remove_file(__sh_path);
                    if let Err(__sh_err) = __sh_res {
                        if __sh_err.kind() == std::io::ErrorKind::NotFound { /* -f: a missing path is not an error */ } else {
                            eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                            __sh_status = 1;
                        }
                    }
                }
                Err(__sh_err) => {
                        if __sh_err.kind() == std::io::ErrorKind::NotFound { /* -f: a missing path is not an error */ } else {
                            eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                            __sh_status = 1;
                        }
                }
            }
        }
        __SH_RC.store(__sh_status, Ordering::Relaxed);
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}