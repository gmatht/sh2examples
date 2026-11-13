use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    let mut __sh_while_ran0 = false;
    let mut __sh_while_last1 = 0;
    while {
print!("{}", "checking...\n");
__SH_RC.store(0, Ordering::SeqCst);
let _ = { let __b = std::path::Path::new("/tmp/somefile").is_file(); __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b };
__SH_RC.load(Ordering::SeqCst) == 0
} {
        __sh_while_ran0 = true;
        println!("{}", "file exists".to_string());
        __SH_RC.store(0, Ordering::SeqCst);
        __sh_while_last1 = __SH_RC.load(Ordering::SeqCst);
        break;
        __sh_while_last1 = __SH_RC.load(Ordering::SeqCst);
    }
    if !__sh_while_ran0 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last1, Ordering::SeqCst); }
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}