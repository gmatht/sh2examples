use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    if { let __b = { use std::os::unix::fs::MetadataExt; match (std::fs::metadata("/path"), std::fs::metadata("/other")) { (Ok(__a), Ok(__b)) => __a.dev() == __b.dev() && __a.ino() == __b.ino(), _ => false } }; __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b } {
        print!("same\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}