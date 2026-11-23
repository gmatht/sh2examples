use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV___sh2_casesubj_0_: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

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

fn __sh_arg(i: usize) -> String {
    __SH_ARGV.lock().unwrap().get(i).cloned().unwrap_or_default()
}

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV___sh2_casesubj_0_.with(|v| *v.borrow_mut() = format!("{}-{}", __sh_arg(0), { let (__sh_ch, __sh_res) = __sh_capture_rc("'uname' '-s'"); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch }));
    if __SHV___sh2_casesubj_0_.with(|v| v.borrow().clone()) == "regex-Linux".to_string() {
        print!("Linux\n");
    } else if __SHV___sh2_casesubj_0_.with(|v| v.borrow().clone()).starts_with("regex-") {
        println!("Other: {}", { let (__sh_ch, __sh_res) = __sh_capture_rc("'uname' '-s'"); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch });
    } else {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}