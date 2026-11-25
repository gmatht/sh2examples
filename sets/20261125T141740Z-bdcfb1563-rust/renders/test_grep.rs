use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_result: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_capture_rc_argv(argv: &[&str]) -> (String, i32) {
    if argv.is_empty() { return (String::new(), 0); }
    let mut c = std::process::Command::new(argv[0]);
    c.args(&argv[1..]);
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

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_result.with(|v| *v.borrow_mut() = { let (__sh_ch, __sh_res) = __sh_capture_rc_argv(&["grep", "pattern", "file.txt"]); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch });
    println!("exit: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}