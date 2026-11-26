use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    let mut __sh2_tc0_ = String::new();
    let _ = "hello".to_string();
    { let __sh_buf = !__sh2_tc0_.clone().is_empty(); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if __sh_buf {
        print!("non-empty\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}