use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    { let __sh_buf = std::env::var("HOME").unwrap_or_default() == std::env::var("HOME").unwrap_or_default() && {
print!("1\n");
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("-\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    { let __sh_buf = format!("{}{}", std::env::var("HOME").unwrap_or_default(), "/Documents") == std::env::var("HOME").unwrap_or_default() && {
print!("2\n");
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("-\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    { let __sh_buf = format!("{}{}", std::env::var("HOME").unwrap_or_default(), "/Documents") == format!("{}/Documents", std::env::var("HOME").unwrap_or_default()) && {
print!("3\n");
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("-\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}