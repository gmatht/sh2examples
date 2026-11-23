use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}