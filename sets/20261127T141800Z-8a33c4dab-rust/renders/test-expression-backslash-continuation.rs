use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn __sh_wq(ws: &[String]) -> String {
    ws.iter().map(|w| __sh_q(w)).collect::<Vec<_>>().join(" ")
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
    let __sh2_tc0_ = String::new();
    let _ = { let (__sh_ch, __sh_res) = __sh_capture_rc(&__sh_wq(&vec!["readlink".to_string(), std::env::var("FILE").unwrap_or_default()])); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch };
    { let __sh_buf = ((!std::fs::symlink_metadata(&std::env::var("FILE").unwrap_or_default()).map(|__sh_mapped| __sh_mapped.file_type().is_symlink()).unwrap_or(false)) && std::path::Path::new(&std::env::var("FILE").unwrap_or_default()).is_dir()) || (std::fs::symlink_metadata(&std::env::var("FILE").unwrap_or_default()).map(|__sh_mapped| __sh_mapped.file_type().is_symlink()).unwrap_or(false) && (__sh2_tc0_.clone() == "target".to_string())); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    print!("parsed OK\n");
    std::process::ExitCode::SUCCESS
}