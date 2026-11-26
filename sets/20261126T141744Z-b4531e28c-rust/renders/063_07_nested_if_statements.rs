use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_array: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

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

fn __sh_regex(text: &str, pat: &str, flags: &str) -> Option<bool> {
    let pat = if flags.contains('i') { format!("(?i){}", pat) } else { pat.to_string() };
    match regex::Regex::new(&pat) {
        Ok(re) => Some(re.is_match(text)),
        Err(_) => None,
    }
}

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    if ({ let __sh_mapped = __sh_regex(&std::env::var("var").unwrap_or_default().to_string(), &"^[0-9]+$".to_string().to_string(), ""); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } } && std::env::var("var").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) > 0) && std::path::Path::new(&std::env::var("file").unwrap_or_default()).is_file() {
        if { let __sh_mapped = __sh_regex(&std::env::var("array[@]").unwrap_or_default().to_string(), &"value".to_string().to_string(), ""); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } } || __SHV_array.with(|v| v.borrow().len() as i64) > 5 {
            if !__sh_capture_rc(&"echo \"$var\" | grep -q \"pattern\"").0.is_empty() {
                print!("Deeply nested condition met\n");
            } else {
            }
        } else {
        }
    } else {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}