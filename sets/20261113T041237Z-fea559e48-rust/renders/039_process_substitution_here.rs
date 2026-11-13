use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

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

fn main() {
    println!("{}", "== Here-string with grep -o ==".to_string());
    let (__m, __r) = __sh_grepmatches("some pattern here", "pattern", "");
    if !__m.is_empty() { println!("{}", __m); }
    __SH_RC.store(__r, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}