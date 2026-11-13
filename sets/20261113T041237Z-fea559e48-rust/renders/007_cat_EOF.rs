use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    print!("alpha\nbeta\ngamma ...\n");
    print!("oyster\nsnapper\nsalmon\n");
    print!("Fin. That is all folks.\n");
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}