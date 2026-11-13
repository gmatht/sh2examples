use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_divisor: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

fn main() {
    print!("=== Number Factorization Examples ===\n");
    factorize("12");
    factorize("28");
    factorize("100");
    factorize("12345");
    print!("Factorization complete!\n");
}

fn factorize(p0: &str) {
    let mut n = format!("{}", p0);
    __SHV_divisor.with(|v| v.set(2));
    let mut factors = "".to_string();
    __SH_RC.store(0, Ordering::SeqCst);
    print!("{}", format!("Factors of {}: ", n.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    let mut __sh_while_ran4 = false;
    let mut __sh_while_last5 = 0;
    while n.clone().trim().parse::<i64>().unwrap_or(0) > 1 {
        __sh_while_ran4 = true;
        let mut __sh_while_ran6 = false;
        let mut __sh_while_last7 = 0;
        while { let __a = n.clone().trim().parse::<i64>().unwrap_or(0); let __b = __SHV_divisor.with(|v| v.get()); if __b == 0 { __SH_ARITH_ERR.store(true, Ordering::SeqCst); 0 } else { __a % __b } } == 0 {
            __sh_while_ran6 = true;
            factors = if !(factors.clone() != "".to_string()) { __SHV_divisor.with(|v| v.get()).to_string() } else { format!("{} * {}", factors.clone(), __SHV_divisor.with(|v| v.get())) };
            __SH_RC.store(0, Ordering::SeqCst);
            n = { let __v = ({ let __a = n.clone().trim().parse::<i64>().unwrap_or(0); let __b = __SHV_divisor.with(|v| v.get()); if __b == 0 { __SH_ARITH_ERR.store(true, Ordering::SeqCst); 0 } else { __a / __b } }).to_string(); let __e = __SH_ARITH_ERR.swap(false, Ordering::SeqCst); __SH_RC.store(if __e { 1 } else { 0 }, Ordering::SeqCst); if __e { String::new() } else { __v } };
            __SH_RC.store(0, Ordering::SeqCst);
            __sh_while_last7 = __SH_RC.load(Ordering::SeqCst);
        }
        if !__sh_while_ran6 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last7, Ordering::SeqCst); }
        __SHV_divisor.with(|v| v.set(__SHV_divisor.with(|v| v.get()) + 1));
        __SH_RC.store(0, Ordering::SeqCst);
        if (__SHV_divisor.with(|v| v.get()) * __SHV_divisor.with(|v| v.get())) > n.clone().trim().parse::<i64>().unwrap_or(0) {
            if n.clone().trim().parse::<i64>().unwrap_or(0) > 1 {
                factors = if !(factors.clone() != "".to_string()) { n.clone() } else { format!("{} * {}", factors.clone(), n.clone()) };
                __SH_RC.store(0, Ordering::SeqCst);
            } else {
                __SH_RC.store(0, Ordering::SeqCst);
            }
            __SH_RC.store(0, Ordering::SeqCst);
            __sh_while_last5 = __SH_RC.load(Ordering::SeqCst);
            break;
        } else {
            __SH_RC.store(0, Ordering::SeqCst);
        }
        __sh_while_last5 = __SH_RC.load(Ordering::SeqCst);
    }
    if !__sh_while_ran4 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last5, Ordering::SeqCst); }
    println!("{}", factors);
    __SH_RC.store(0, Ordering::SeqCst);
}