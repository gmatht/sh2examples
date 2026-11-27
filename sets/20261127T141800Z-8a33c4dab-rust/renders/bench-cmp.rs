use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    { let __sh_buf = "abc" == "abc"; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    println!("eq={}", __SH_RC.load(Ordering::Relaxed));
    { let __sh_buf = "abc" != "xyz"; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    println!("ne={}", __SH_RC.load(Ordering::Relaxed));
    { let __sh_buf = 5 < 10; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    println!("lt={}", __SH_RC.load(Ordering::Relaxed));
    __SH_RC.store(0, Ordering::Relaxed);
    std::process::ExitCode::SUCCESS
}