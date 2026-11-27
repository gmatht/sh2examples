use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    let mut i: i64;
    i = 0;
    let mut __sh_while_ran0 = false;
    let mut __sh_while_last1 = 0;
    while i < 10 {
        __sh_while_ran0 = true;
        print!("{}", i);
        i = i + 1;
        __SH_RC.store(0, Ordering::Relaxed);
        __sh_while_last1 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran0 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last1, Ordering::Relaxed); }
    println!();
    std::process::ExitCode::SUCCESS
}