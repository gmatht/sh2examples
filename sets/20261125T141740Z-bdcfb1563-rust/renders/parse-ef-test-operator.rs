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
    if {
{
    { let __sh_buf = __sh_samefile(&std::env::var("A").unwrap_or_default(), &std::env::var("B").unwrap_or_default()); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
}
__SH_RC.load(Ordering::Relaxed) == 0
} {
        print!("same file\n");
        __SH_RC.store(0, Ordering::Relaxed);
    } else {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}