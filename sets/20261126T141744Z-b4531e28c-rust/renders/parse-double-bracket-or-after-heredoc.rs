use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    if std::path::Path::new("/path1").is_dir() || std::path::Path::new("/path2").is_dir() {
        print!("found\n");
    } else {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    println!("done: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}