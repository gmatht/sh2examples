use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    let mut __sh_while_ran0 = false;
    let mut __sh_while_last1 = 0;
    loop {
        __sh_while_ran0 = true;
        print!("loop\n");
        __SH_RC.store(0, Ordering::Relaxed);
        __sh_while_last1 = __SH_RC.load(Ordering::Relaxed);
        break;
    }
    if !__sh_while_ran0 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last1, Ordering::Relaxed); }
    println!("status: {}", "ok");
    std::process::ExitCode::SUCCESS
}