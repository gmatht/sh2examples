use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    if std::path::Path::new("/etc/config").is_file() {
        { let __sh_source = std::fs::read_to_string("/etc/config").unwrap_or_default(); for __line in __sh_source.lines() { if let Some((__sh_num, __sh_piece)) = __line.trim().split_once('=') { std::env::set_var(__sh_num, __sh_piece.trim()); } } }
        { let __sh_source = std::fs::read_to_string("/etc/profile").unwrap_or_default(); for __line in __sh_source.lines() { if let Some((__sh_num, __sh_piece)) = __line.trim().split_once('=') { std::env::set_var(__sh_num, __sh_piece.trim()); } } }
    } else {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    println!("exit: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}