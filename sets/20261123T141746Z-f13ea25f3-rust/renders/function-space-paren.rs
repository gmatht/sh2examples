use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
}

fn main() -> std::process::ExitCode {
    println!("{}", my_func());
    __SH_RC.store(0, Ordering::Relaxed);
    println!("done: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn my_func() -> String {
    let __sh_value = "hello".to_string(); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed); return __sh_value;
}