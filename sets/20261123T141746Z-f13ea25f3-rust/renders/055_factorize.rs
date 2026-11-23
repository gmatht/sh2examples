use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn __sh_div(a: i64, b: i64) -> i64 {
    if b == 0 { __SH_ARITH_ERR.store(true, Ordering::Relaxed); 0 } else { a / b }
}

fn __sh_mod(a: i64, b: i64) -> i64 {
    if b == 0 { __SH_ARITH_ERR.store(true, Ordering::Relaxed); 0 } else { a % b }
}

fn main() -> std::process::ExitCode {
    print!("=== Number Factorization Examples ===\n");
    factorize("12");
    factorize("28");
    factorize("100");
    factorize("12345");
    print!("Factorization complete!\n");
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn factorize(p0: &str) {
    let mut n = format!("{}", p0);
    let mut divisor = 2;
    let mut factors = "".to_string();
    __SH_RC.store(0, Ordering::Relaxed);
    print!("Factors of {}: ", n.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_while_ran4 = false;
    let mut __sh_while_last5 = 0;
    while n.trim().parse::<i64>().unwrap_or(0) > 1 {
        __sh_while_ran4 = true;
        let mut __sh_while_ran6 = false;
        let mut __sh_while_last7 = 0;
        while __sh_mod(n.trim().parse::<i64>().unwrap_or(0),divisor) == 0 {
            __sh_while_ran6 = true;
            factors = if factors.clone() == "".to_string() { divisor.to_string() } else { format!("{} * {}", factors.clone(), divisor) };
            __SH_RC.store(0, Ordering::Relaxed);
            n = { let __sh_value = (__sh_div(n.trim().parse::<i64>().unwrap_or(0),divisor)).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } };
            __SH_RC.store(0, Ordering::Relaxed);
            __sh_while_last7 = __SH_RC.load(Ordering::Relaxed);
        }
        if !__sh_while_ran6 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last7, Ordering::Relaxed); }
        divisor = divisor + 1;
        __SH_RC.store(0, Ordering::Relaxed);
        if (divisor * divisor) > n.trim().parse::<i64>().unwrap_or(0) {
            if n.trim().parse::<i64>().unwrap_or(0) > 1 {
                factors = if factors.clone() == "".to_string() { n.clone() } else { format!("{} * {}", factors.clone(), n.clone()) };
                __SH_RC.store(0, Ordering::Relaxed);
            } else {
                __SH_RC.store(0, Ordering::Relaxed);
            }
            __SH_RC.store(0, Ordering::Relaxed);
            __sh_while_last5 = __SH_RC.load(Ordering::Relaxed);
            break;
        } else {
            __SH_RC.store(0, Ordering::Relaxed);
        }
        __sh_while_last5 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran4 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last5, Ordering::Relaxed); }
    println!("{}", factors.clone());
    __SH_RC.store(0, Ordering::Relaxed);
}