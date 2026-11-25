use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    print!("Testing complex here-documents...\n");
    print!("This is a test line\nThis is not a test line\nThis is another test line\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("done: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}