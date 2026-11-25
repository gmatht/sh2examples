use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    c.stdin(std::process::Stdio::inherit());
    let Ok(mut ch) = c.spawn() else { return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let mut s = String::from_utf8_lossy(&out).to_string();
    while s.ends_with('\n') { s.pop(); }
    (s, rc)
}

fn main() -> std::process::ExitCode {
    let all_interfaces = { let (__sh_ch, __sh_res) = __sh_capture_rc(&format!("{} < {}", "'sed' '-n' 's/.*//p'", __sh_q("/dev/null"))); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch };
    println!("{}", all_interfaces.clone());
    std::process::ExitCode::SUCCESS
}