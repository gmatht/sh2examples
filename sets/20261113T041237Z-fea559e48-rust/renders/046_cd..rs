use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    let _ = { let __d = "..".to_string(); let __r = std::env::set_current_dir(&__d); if __r.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __r.is_ok() { 0 } else { 1 }, Ordering::SeqCst); __r.is_ok() };
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("ls").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}