use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    print!("== Advanced brace expansion ==\n");
    print!("a1 a2 a3 b1 b2 b3 c1 c2 c3\n");
    print!("1 3 5 7 9\n");
    print!("a d g j m p s v y\n");
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}