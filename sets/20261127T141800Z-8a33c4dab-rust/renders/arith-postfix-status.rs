use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() -> std::process::ExitCode {
    let mut i = 0;
    __SH_RC.store(0, Ordering::Relaxed);
    let __sh_arith_value0 = { let __sh_out = i; i = i +1; __sh_out };
    let __sh_num1 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value0 };
    __SH_RC.store(if __sh_num1 != 0 { 0 } else { 1 }, Ordering::Relaxed);
    println!("post0={}", __SH_RC.load(Ordering::Relaxed));
    let __sh_arith_value2 = { i = i +1; i };
    let __sh_num3 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value2 };
    __SH_RC.store(if __sh_num3 != 0 { 0 } else { 1 }, Ordering::Relaxed);
    println!("pre={}", __SH_RC.load(Ordering::Relaxed));
    i = 5;
    __SH_RC.store(0, Ordering::Relaxed);
    let __sh_arith_value4 = { let __sh_out = i; let _ = i +1; __sh_out };
    let __sh_num5 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value4 };
    __SH_RC.store(if __sh_num5 != 0 { 0 } else { 1 }, Ordering::Relaxed);
    println!("post5={}", __SH_RC.load(Ordering::Relaxed));
    __SH_RC.store(0, Ordering::Relaxed);
    std::process::ExitCode::SUCCESS
}