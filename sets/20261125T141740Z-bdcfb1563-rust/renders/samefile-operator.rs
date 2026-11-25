use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn __sh_samefile(a: &str, b: &str) -> bool {
    use std::os::unix::fs::MetadataExt;
    match (std::fs::metadata(a), std::fs::metadata(b)) {
        (Ok(ma), Ok(mb)) => ma.dev() == mb.dev() && ma.ino() == mb.ino(),
        _ => false,
    }
}

fn main() -> std::process::ExitCode {
    { let __sh_buf = __sh_samefile("/path", "/other"); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if __sh_buf {
        print!("same\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    println!("exit: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}