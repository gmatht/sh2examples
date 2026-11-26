use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn __sh_spawn(cmd: &str, input: Option<&[u8]>) -> i32 {
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
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
    if want_out { c.stdout(std::process::Stdio::piped()); }
    if input.is_none() {
        if let Some(p) = &stdin_path {
            if let Ok(f) = std::fs::File::open(p) {
                c.stdin(std::process::Stdio::from(f));
            }
        }
    }
    if input.is_some() { c.stdin(std::process::Stdio::piped()); }
    let Ok(mut ch) = c.spawn() else { return 1 };
    if let Some(data) = input {
        if let Some(mut si) = ch.stdin.take() { let _ = std::io::Write::write_all(&mut si, &data); }
    }
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    if !out.is_empty() { __sh_sink(&out); }
    rc
}

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

fn main() -> std::process::ExitCode {
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{}", __e) } };
        for __sh_path in &vec!["src.txt".to_string(), "dest.txt".to_string()] {
            match std::fs::OpenOptions::new().create(true).append(true).open(__sh_path) {
                Ok(__sh_field) => { let _ = __sh_field.set_modified(std::time::SystemTime::now()); }
                // `touch DIR` is a legal time update: append-open refuses a
                // directory, so the directory is opened read-only for the same
                // `set_modified` (bash succeeds here; without this the arm would
                // print a diagnostic bash never prints).
                Err(__sh_err) if __sh_err.kind() == std::io::ErrorKind::IsADirectory => {
                    match std::fs::File::open(__sh_path) {
                        Ok(__sh_field) => { let _ = __sh_field.set_modified(std::time::SystemTime::now()); }
                        Err(__sh_err) => {
                            eprintln!("touch: cannot touch '{}': {}", __sh_path, err_text(&__sh_err));
                            __sh_status = 1;
                        }
                    }
                }
                Err(__sh_err) => {
                    eprintln!("touch: cannot touch '{}': {}", __sh_path, err_text(&__sh_err));
                    __sh_status = 1;
                }
            }
        }
        __SH_RC.store(__sh_status, Ordering::Relaxed);
    }
    __SH_RC.store(__sh_spawn(&format!("{} 2> {}", "'mv' '-Z' 'src.txt' 'dest.txt'", __sh_q("/dev/null")), None), Ordering::Relaxed);
    println!("mv rc: {}", __SH_RC.load(Ordering::Relaxed));
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{}", __e) } };
        for __sh_path in &vec!["src.txt".to_string(), "dest.txt".to_string()] {
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