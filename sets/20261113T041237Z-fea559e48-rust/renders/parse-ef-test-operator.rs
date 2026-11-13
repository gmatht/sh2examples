use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    if {
{
    let _ = { let __b = { use std::os::unix::fs::MetadataExt; match (std::fs::metadata(&std::env::var("A").unwrap_or_default()), std::fs::metadata(&std::env::var("B").unwrap_or_default())) { (Ok(__a), Ok(__b)) => __a.dev() == __b.dev() && __a.ino() == __b.ino(), _ => false } }; __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b };
}
__SH_RC.load(Ordering::SeqCst) == 0
} {
        println!("{}", "same file".to_string());
        __SH_RC.store(0, Ordering::SeqCst);
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}