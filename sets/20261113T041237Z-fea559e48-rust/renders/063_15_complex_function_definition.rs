use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() {
    complex_func("3".to_string().to_string(), "7".to_string().to_string());
    complex_func("10".to_string().to_string(), "20".to_string().to_string());
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}

fn complex_func(p0: String, p1: String) {
    let x = p0.clone();
    let y = p1.clone();
    __SH_RC.store(0, Ordering::SeqCst);
    let result = { let __v = ((x.clone().trim().parse::<i64>().unwrap_or(0) + y.clone().trim().parse::<i64>().unwrap_or(0))).to_string(); if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { String::new() } else { __v } };
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("Sum: {}", result.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("Args: {} {}", x.clone(), y.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
}