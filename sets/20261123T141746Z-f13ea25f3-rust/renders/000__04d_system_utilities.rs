use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

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
    print!("=== System Utilities ===\n");
    let formatted_date = { let (__sh_ch, __sh_res) = __sh_capture_rc("'date' '+%Y-%m-%d'"); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch };
    println!("Formatted date: {}", formatted_date.clone());
    println!("Sleeping for {} seconds...", "1");
    std::thread::sleep(std::time::Duration::from_secs_f64(("1".to_string()).trim().parse::<f64>().unwrap_or(0.0).max(0.0)));
    print!("Yes command result:\n");
    print!("Hello\nHello\nHello\n");
    print!("=== System Utilities Complete ===\n");
    std::process::ExitCode::SUCCESS
}