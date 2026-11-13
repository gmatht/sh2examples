use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    let __sh_case0 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() };
    if (__sh_case0 == "test") {
        if {
__SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::fs::File::create("/dev/null") { Ok(__rf) => match std::process::Command::new("cmd").stdout(__rf).status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } }, Err(_) => 1 } }, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
} {
            print!("{}", "ok\n");
            __SH_RC.store(0, Ordering::SeqCst);
        }
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}