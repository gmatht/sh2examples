use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn __sh_arg(i: usize) -> String {
    __SH_ARGV.lock().unwrap().get(i).cloned().unwrap_or_default()
}

fn main() -> std::process::ExitCode {
    if __sh_arg(0) == "foo".to_string() {
        __SH_RC.store(0, Ordering::Relaxed); return std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8);
    } else if __sh_arg(0) == "bar".to_string() {
        return std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8);
    } else {
        print!("unknown\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}