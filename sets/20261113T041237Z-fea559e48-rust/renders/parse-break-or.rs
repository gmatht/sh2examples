use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    while true {
        if !{ __SH_RC.store(0, Ordering::SeqCst); break; false } {
            std::process::exit(1);
        }
    }
    print!("{}", "parsed OK\n".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}