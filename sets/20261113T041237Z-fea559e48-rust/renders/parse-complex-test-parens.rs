use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    c.stdin(std::process::Stdio::inherit());
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

fn main() {
    if (((!std::fs::symlink_metadata("/path").map(|m| m.file_type().is_symlink()).unwrap_or(false)) && std::path::Path::new("/path").is_dir()) && std::path::Path::new("/path/file").is_file()) || (std::fs::symlink_metadata("/path").map(|m| m.file_type().is_symlink()).unwrap_or(false) && (__sh_capture_rc(&"readlink \"/path\"").0 == "target".to_string())) && { let __r = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("dpkg").arg("--compare-versions").arg("--").arg("1.0").arg("le").arg("2.0").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }; __SH_RC.store(__r, Ordering::SeqCst); __r == 0 } {
        print!("complex condition met\n");
        __SH_RC.store(0, Ordering::SeqCst);
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}