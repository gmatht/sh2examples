use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

fn main() {
    println!("{}", mul_xy("3".to_string().to_string(), "4".to_string().to_string()));
}

fn mul_xy(p0: String, p1: String) -> String {
    let x = p0.clone();
    let y = p1.clone();
    __SH_RC.store(0, Ordering::SeqCst);
    let __v = { let __v = ((x.clone().trim().parse::<i64>().unwrap_or(0) * y.clone().trim().parse::<i64>().unwrap_or(0))).to_string(); if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { String::new() } else { __v } }; *__SH_RET.lock().unwrap() = __v.clone(); return __v;
}