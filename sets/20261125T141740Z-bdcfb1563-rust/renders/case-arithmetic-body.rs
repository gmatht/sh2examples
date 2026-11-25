use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    if __SH_RC.load(Ordering::Relaxed).to_string() == "0".to_string() {
        println!("{}", (1i64 + 2i64));
    } else {
        print!("other\n");
    }
    std::process::ExitCode::SUCCESS
}