use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("/tmp/paste_a.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "a\n".as_bytes());
    }
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("/tmp/paste_b.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "b\n".as_bytes());
    }
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("paste").arg("/tmp/paste_a.txt").arg("/tmp/paste_b.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg("/tmp/paste_a.txt").arg("/tmp/paste_b.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}