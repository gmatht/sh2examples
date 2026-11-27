use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() -> std::process::ExitCode {
    let mut __n: i64;
    let mut i = 1;
    __n = 0;
    let mut __sh_while_ran0 = false;
    let mut __sh_while_last1 = 0;
    while __n < 1000 {
        __sh_while_ran0 = true;
        let __sh_arith_value2 = { let __sh_out = i; i = i +1; __sh_out };
        let __sh_num3 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value2 };
        __SH_RC.store(if __sh_num3 != 0 { 0 } else { 1 }, Ordering::Relaxed);
        __n = __n + 1;
        __SH_RC.store(0, Ordering::Relaxed);
        __sh_while_last1 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran0 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last1, Ordering::Relaxed); }
    println!("increment={}", i);
    std::process::ExitCode::SUCCESS
}