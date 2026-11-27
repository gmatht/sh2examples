use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    for _i in [1, 2, 3] {
        { let __sh_buf = _i == 2; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if __sh_buf {
            println!("hit={}", _i);
            __SH_RC.store(0, Ordering::Relaxed);
        }
        }
    }
    { let __sh_buf = __SH_RC.load(Ordering::Relaxed).to_string() == "0".to_string(); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if __sh_buf {
        print!("status-zero\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_buf = __SH_RC.load(Ordering::Relaxed).to_string() == "0".to_string(); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if __sh_buf {
        print!("status-zero2\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}