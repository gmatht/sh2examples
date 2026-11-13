use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    let __sh_case0 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() };
    if (__sh_case0 == "-h") || (__sh_case0 == "--help") {
        __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("usage").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
        std::process::exit(0);
    } else if ((__sh_case0 == "--version")) {
        println!("{}", std::env::var("VERSION").unwrap_or_default());
        std::process::exit(0);
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}