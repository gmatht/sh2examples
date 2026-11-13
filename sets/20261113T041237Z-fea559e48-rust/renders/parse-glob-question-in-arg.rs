use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    let f = "/var/lib/foo";
    if std::path::Path::new(&f).is_dir() {
        __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg(format!("{}/??\\:??\\:??", "/var/lib/foo".to_string())).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    }
    print!("{}", format!("{}=[{}]\n", "f".to_string(), "/var/lib/foo".to_string()));
}