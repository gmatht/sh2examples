use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
}

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    if std::path::Path::new("file.txt").is_file() {
        print!("File exists\n");
    } else {
        print!("File does not exist\n");
    }
    for __sh_lv1 in [1, 2, 3, 4, 5] {
        __SHV_i.with(|v| v.set(__sh_lv1));
        println!("Number: {}", __sh_lv1);
    }
    while __SHV_i.with(|v| v.get()) < 10 {
        println!("Counter: {}", __SHV_i.with(|v| v.get()));
        __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 1));
    }
    println!("{}", greet("World".to_string()));
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn greet(p0: String) -> String {
    let __sh_value = format!("Hello, {}!", p0.clone()); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
    return __sh_value;
}