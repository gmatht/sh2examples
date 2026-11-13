use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    f();
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}

fn g() {
    {
        print!("{}", "hi\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
}

fn f() {
    g();
}