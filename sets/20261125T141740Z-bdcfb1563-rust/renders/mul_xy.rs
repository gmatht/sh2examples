use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
}

fn main() -> std::process::ExitCode {
    println!("{}", mul_xy("3".to_string(), "4".to_string()));
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn mul_xy(p0: String, p1: String) -> String {
    let x = p0.clone();
    let y = p1.clone();
    let __sh_value = { let __sh_value = ((x.trim().parse::<i64>().unwrap_or(0) * y.trim().parse::<i64>().unwrap_or(0))).to_string(); if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { String::new() } else { __sh_value } }; *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
    return __sh_value;
}