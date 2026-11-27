use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn __sh_capture_rc_argv(argv: &[&str]) -> (String, i32) {
    if argv.is_empty() { return (String::new(), 0); }
    let mut c = std::process::Command::new(argv[0]);
    c.args(&argv[1..]);
    c.stdout(std::process::Stdio::piped());
    let Ok(mut ch) = c.spawn() else { return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let s = String::from_utf8_lossy(&out).trim_end_matches('\n').to_string();
    (s, rc)
}

fn main() -> std::process::ExitCode {
    let existing = { let (__sh_ch, __sh_res) = __sh_capture_rc_argv(&["readlink", "-e", "/usr/bin/vi"]); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch };
    let missing = { let (__sh_ch, __sh_res) = __sh_capture_rc_argv(&["readlink", "-m", "/nonexistent/path"]); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch };
    let full = { let (__sh_ch, __sh_res) = __sh_capture_rc_argv(&["readlink", "-f", "/usr/bin/python3"]); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch };
    println!("Existing: {}", existing.clone());
    println!("Missing:  {}", missing.clone());
    println!("Full:     {}", full.clone());
    std::process::ExitCode::SUCCESS
}