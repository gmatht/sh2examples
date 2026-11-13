use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    print!("Test non-UTF-8:\n");
    print!("{}", (&unsafe { String::from_utf8_unchecked(vec![0x61,0x63,0x63,0xE9,0x6E,0x74,0x0A,]) }));
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}