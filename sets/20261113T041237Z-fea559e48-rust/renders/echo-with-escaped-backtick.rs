use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    println!("{}", format!("Invalid configuration `{}\\': more than four components", { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() }));
    std::process::exit(1);
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}