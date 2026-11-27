use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_file: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
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
    get_file_size("test_simple_function.sh".to_string());
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn get_file_size(p0: String) {
    let __sh_loc_file: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
    __SHV_file.with(|v| *v.borrow_mut() = format!("{}", p0.clone()));
    let size = { let __sh_split_s: String = { let (__sh_ch, __sh_res) = __sh_capture_rc(&format!("{} < {}", "'wc' '-c'", __sh_q(&__SHV_file.with(|v| v.borrow().clone())))); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch }; __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }.join(" ");
    println!("File {} has {} bytes", __SHV_file.with(|v| v.borrow().clone()), size.clone());
    match __sh_loc_file.take() { Some(__sh_loc) => { __SHV_file.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
}