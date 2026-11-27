use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    if __SH_RC.load(Ordering::Relaxed).to_string().trim().parse::<i64>().unwrap_or(0) == 0 {
        print!("success\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}