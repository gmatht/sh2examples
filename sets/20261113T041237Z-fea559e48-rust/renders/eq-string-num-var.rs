use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    for i in [1, 2, 3] {
        if !(i != 2) {
            println!("{}", format!("hit={}", i));
            __SH_RC.store(0, Ordering::SeqCst);
        }
    }
    if { let __b = __SH_RC.load(Ordering::SeqCst).to_string() == "0".to_string(); __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b } {
        print!("status-zero\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    __SH_RC.store(0, Ordering::SeqCst);
    if { let __b = __SH_RC.load(Ordering::SeqCst).to_string() == "0".to_string(); __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b } {
        print!("status-zero2\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}