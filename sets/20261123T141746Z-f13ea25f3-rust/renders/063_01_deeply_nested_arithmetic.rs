use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn __sh_pow(a: i64, b: i64) -> i64 {
    a.wrapping_pow(b.clamp(0, u32::MAX as i64) as u32)
}

fn __sh_div(a: i64, b: i64) -> i64 {
    if b == 0 { __SH_ARITH_ERR.store(true, Ordering::Relaxed); 0 } else { a / b }
}

fn __sh_mod(a: i64, b: i64) -> i64 {
    if b == 0 { __SH_ARITH_ERR.store(true, Ordering::Relaxed); 0 } else { a % b }
}

fn main() -> std::process::ExitCode {
    let mut a = 0i64;
    let mut b = 0i64;
    let mut c = 0i64;
    let mut d = 0i64;
    let mut e = 0i64;
    let mut f = 0i64;
    let mut g = 0i64;
    let mut h = 0i64;
    let mut i = 0i64;
    let mut j = 0i64;
    let mut k = 0i64;
    let mut l = 0i64;
    let mut m = 0i64;
    let mut n = 0i64;
    a = 10;
    b = 3;
    c = 8;
    d = 2;
    e = 7;
    f = 2;
    g = 2;
    h = 3;
    i = 4;
    j = 1;
    k = 6;
    l = 3;
    m = 5;
    n = 2;
    let result = { let __sh_value = ((((__sh_div((a + b) * (c - d),__sh_mod(e,f)) + __sh_pow(g,h)) - (i << j)) | ((k & l) ^ (m | n)))).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } };
    println!("Deeply nested arithmetic result: {}", result.clone());
    std::process::ExitCode::SUCCESS
}