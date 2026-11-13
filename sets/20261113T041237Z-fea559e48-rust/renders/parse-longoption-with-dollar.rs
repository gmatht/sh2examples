use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    #[allow(non_snake_case)] let mut X = "test";
    __SH_RC.store(0, Ordering::SeqCst);
    if !{
println!("{}", format!("--x={}", X));
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
} {
        let _ = { let __v = "0".to_string(); X = __v.clone(); __v };
        __SH_RC.store(0, Ordering::SeqCst);
    }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}