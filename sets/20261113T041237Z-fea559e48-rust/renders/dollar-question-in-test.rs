use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    __SH_RC.store(0, Ordering::SeqCst);
    if __SH_RC.load(Ordering::SeqCst).to_string().trim().parse::<i64>().unwrap_or(0) == 0 {
        print!("ok\n");
        __SH_RC.store(0, Ordering::SeqCst);
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
}