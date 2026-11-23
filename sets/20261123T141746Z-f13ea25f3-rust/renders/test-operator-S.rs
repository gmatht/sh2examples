use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    if std::fs::metadata("/dev/null").map(|__sh_mapped| { use std::os::unix::fs::MetadataExt; (__sh_mapped.mode() & 0o140000) == 0o140000 }).unwrap_or(false) {
        print!("is a socket\n");
    }
    if std::fs::metadata("/dev/null").map(|__sh_mapped| { use std::os::unix::fs::MetadataExt; (__sh_mapped.mode() & 0o140000) == 0o140000 }).unwrap_or(false) {
        print!("is a socket\n");
        __SH_RC.store(0, Ordering::Relaxed);
    } else {
        __SH_RC.store(0, Ordering::Relaxed);
    }
    { let __sh_buf = std::fs::metadata("/dev/null").map(|__sh_mapped| { use std::os::unix::fs::MetadataExt; (__sh_mapped.mode() & 0o140000) == 0o140000 }).unwrap_or(false) && {
print!("is_socket: yes\n");
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("is_socket: no\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    println!("done: {}", __SH_RC.load(Ordering::Relaxed));
    __SH_RC.store(0, Ordering::Relaxed);
    std::process::ExitCode::SUCCESS
}