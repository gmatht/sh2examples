use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    for _i in [1, 2, 3] {
        { let __sh_buf = _i == 2; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
            continue;
        }
        }
        println!("{}", _i);
        __SH_RC.store(0, Ordering::Relaxed);
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}