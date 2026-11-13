use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    __SH_RC.store(0, Ordering::SeqCst);
    let _ = (if (!{ let __r = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("-q").arg("foo").arg("/dev/null").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }; __SH_RC.store(__r, Ordering::SeqCst); __r == 0 }) { "true".to_string() } else { "false".to_string() });
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}