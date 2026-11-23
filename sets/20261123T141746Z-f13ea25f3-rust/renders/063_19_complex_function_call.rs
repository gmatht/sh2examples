use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
}

fn main() -> std::process::ExitCode {
    println!("{}", { let __sh_operand: Vec<String> = vec!["--long-option=value with spaces".to_string(), "--array-option".to_string(), "item1".to_string(), "item2".to_string(), "item3".to_string(), "--flag".to_string(), "positional argument".to_string(), { let __sh_value = std::env::var("var").unwrap_or_default(); if __sh_value.is_empty() { "default".to_string() } else { __sh_value.to_string() } }, "computed".to_string()]; complex_function(&__sh_operand) });
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn complex_function(__argv: &[String]) -> String {
    let __sh_value = format!("fn args: {}", __argv.to_vec().join(" ")); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed); return __sh_value;
}