use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    let mut __sh_while_ran0 = false;
    let mut __sh_while_last1 = 0;
    while !{
{
    let _ = { let _ = {
{
__SH_RC.store(0, Ordering::SeqCst);
let _ = std::io::Write::write_all(&mut std::fs::OpenOptions::new().append(true).create(true).open("/tmp/parse_error_block_test").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "".as_bytes());
}
__SH_RC.load(Ordering::SeqCst) == 0
}; String::new() };
    __SH_RC.store(0, Ordering::SeqCst);
}
__SH_RC.load(Ordering::SeqCst) == 0
} {
        __sh_while_ran0 = true;
        println!("{}", "waiting".to_string());
        __SH_RC.store(0, Ordering::SeqCst);
        __sh_while_last1 = __SH_RC.load(Ordering::SeqCst);
    }
    if !__sh_while_ran0 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last1, Ordering::SeqCst); }
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}