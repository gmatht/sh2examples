use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

fn main() {
    let result = {
    let (__c, __r) = { let __sc = std::process::Command::new("cat").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __sc { Ok(mut __sc) => { if let Some(mut __ci) = __sc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, "I can't stop this feeling\n".to_string().as_bytes()); } let __so = __sc.wait_with_output(); match __so { Ok(__so) => { let mut __ss = String::from_utf8_lossy(&__so.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __so.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
};
    print!("{}", format!("heredoc content=[{}]\n", result.clone()));
}