use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn __sh_div(a: i64, b: i64) -> i64 {
    if b == 0 { __SH_ARITH_ERR.store(true, Ordering::Relaxed); 0 } else { a / b }
}

fn main() -> std::process::ExitCode {
    let a = String::new();
    let b = String::new();
    let c = String::new();
    let d = String::new();
    let x = { let __sh_value = (__sh_div(a.trim().parse::<i64>().unwrap_or(0) + b.trim().parse::<i64>().unwrap_or(0),c.trim().parse::<i64>().unwrap_or(0) + d.trim().parse::<i64>().unwrap_or(0))).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } };
    print!("x=[{}]\n", x.clone());
    std::process::ExitCode::SUCCESS
}