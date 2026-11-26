use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() -> std::process::ExitCode {
    complex_func("3".to_string(), "7".to_string());
    complex_func("10".to_string(), "20".to_string());
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn complex_func(p0: String, p1: String) {
    let x = p0.clone();
    let y = p1.clone();
    let result = { let __sh_value = ((x.trim().parse::<i64>().unwrap_or(0) + y.trim().parse::<i64>().unwrap_or(0))).to_string(); if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { String::new() } else { __sh_value } };
    println!("Sum: {}", result.clone());
    println!("Args: {} {}", x.clone(), y.clone());
}