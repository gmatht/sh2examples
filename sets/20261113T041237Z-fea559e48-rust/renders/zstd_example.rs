use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::fs::File::create("/tmp/rp2.fw") { Ok(__rf) => match std::process::Command::new("zstd").arg("-dc").arg("/usr/lib/firmware/rp2.fw.zst").stdout(__rf).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } }, Err(_) => 1 } }, Ordering::SeqCst);
    println!("{}", "Decompressed rp2 firmware".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}