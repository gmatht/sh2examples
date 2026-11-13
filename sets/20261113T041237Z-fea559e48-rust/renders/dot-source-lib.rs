use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    {
    let _ = std::io::Write::write_all(&mut std::fs::File::create("lib_tmp.sh").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "shared_val=secret\n".as_bytes());
    }
    { let __src = std::fs::read_to_string("./lib_tmp.sh").unwrap_or_default(); for __line in __src.lines() { if let Some((__n, __val)) = __line.trim().split_once('=') { std::env::set_var(__n, __val.trim()); } } }
    println!("{}", format!("shared={}", std::env::var("shared_val").unwrap_or_default()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg("lib_tmp.sh").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}