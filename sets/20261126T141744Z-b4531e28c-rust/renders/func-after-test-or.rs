use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
}

fn main() -> std::process::ExitCode {
    #[allow(non_snake_case)] let mut VAR = String::new();
    { let __sh_buf = VAR.clone() != "".to_string(); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        { let __sh_value = "default".to_string(); let _ = "default".to_string(); __sh_value };
    }
    }
    println!("{}", my_func());
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn my_func() -> String {
    let __sh_value = "hello".to_string(); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
    return __sh_value;
}