use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    let mut __sh_while_ran0 = false;
    let mut __sh_while_last1 = 0;
    while !{
{
    let _ = { let _ = {
{ let mut __sh_field = std::fs::OpenOptions::new().append(true).create(true).open("/tmp/parse_error_block_test").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f }); use std::io::Write; let _ = __sh_field.write_all("".to_string().as_bytes()); }
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; String::new() };
    __SH_RC.store(0, Ordering::Relaxed);
}
__SH_RC.load(Ordering::Relaxed) == 0
} {
        __sh_while_ran0 = true;
        print!("waiting\n");
        __SH_RC.store(0, Ordering::Relaxed);
        __sh_while_last1 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran0 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last1, Ordering::Relaxed); }
    println!("exit: {}", __SH_RC.load(Ordering::Relaxed));
    __SH_RC.store(0, Ordering::Relaxed);
    std::process::ExitCode::SUCCESS
}