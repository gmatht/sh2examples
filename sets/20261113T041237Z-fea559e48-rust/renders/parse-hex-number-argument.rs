use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    print!("0x1234\n");
    let hex_val = 255;
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("hex_val: {}", hex_val));
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("done: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}