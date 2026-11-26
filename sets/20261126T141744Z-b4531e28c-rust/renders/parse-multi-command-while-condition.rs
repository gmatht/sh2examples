use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    let mut __sh_while_ran0 = false;
    let mut __sh_while_last1 = 0;
    while {
print!("checking...\n");
__SH_RC.store(0, Ordering::Relaxed);
{ let __sh_buf = std::path::Path::new("/tmp/somefile").is_file(); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
__SH_RC.load(Ordering::Relaxed) == 0
} {
        __sh_while_ran0 = true;
        print!("file exists\n");
        __SH_RC.store(0, Ordering::Relaxed);
        __sh_while_last1 = __SH_RC.load(Ordering::Relaxed);
        break;
    }
    if !__sh_while_ran0 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last1, Ordering::Relaxed); }
    println!("exit: {}", __SH_RC.load(Ordering::Relaxed));
    __SH_RC.store(0, Ordering::Relaxed);
    std::process::ExitCode::SUCCESS
}