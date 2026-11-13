use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    if std::path::Path::new("/etc/config").is_file() {
        { let __src = std::fs::read_to_string("/etc/config").unwrap_or_default(); for __line in __src.lines() { if let Some((__n, __val)) = __line.trim().split_once('=') { std::env::set_var(__n, __val.trim()); } } }
        { let __src = std::fs::read_to_string("/etc/profile").unwrap_or_default(); for __line in __src.lines() { if let Some((__n, __val)) = __line.trim().split_once('=') { std::env::set_var(__n, __val.trim()); } } }
        __SH_RC.store(0, Ordering::SeqCst);
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}