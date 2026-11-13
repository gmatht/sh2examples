use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    if std::path::Path::new("file.txt").is_file() {
        print!("File exists\n");
        __SH_RC.store(0, Ordering::SeqCst);
    } else {
        print!("File does not exist\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}