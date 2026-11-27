use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    #[allow(non_snake_case)] let X = "test";
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_buf = {
println!("--x={}", X);
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        { let __sh_value = "0".to_string(); let _ = "0".to_string(); __sh_value };
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}