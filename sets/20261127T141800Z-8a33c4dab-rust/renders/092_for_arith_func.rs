use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_n: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
}

fn main() -> std::process::ExitCode {
    println!("{}", factorial("5".to_string()));
    println!("{}", factorial("6".to_string()));
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn factorial(p0: String) -> String {
    let mut i = 0i64;
    let __sh_loc_n: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
    __SHV_n.with(|v| *v.borrow_mut() = format!("{}", p0.clone()));
    let mut result = 1;
    let __sh_arith_value6 = { let __sh_value = 2; i = __sh_value; __sh_value };
    let __sh_num7 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value6 };
    __SH_RC.store(if __sh_num7 != 0 { 0 } else { 1 }, Ordering::Relaxed);
    let mut __sh_while_ran8 = false;
    let mut __sh_while_last9 = 0;
    while i <= __SHV_n.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) {
        __sh_while_ran8 = true;
        result = result * i;
        __SH_RC.store(0, Ordering::Relaxed);
        let __sh_arith_value10 = { let __sh_out = i; i = i +1; __sh_out };
        let __sh_num11 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value10 };
        __SH_RC.store(if __sh_num11 != 0 { 0 } else { 1 }, Ordering::Relaxed);
        __sh_while_last9 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran8 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last9, Ordering::Relaxed); }
    let __sh_value = result.to_string(); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
    match __sh_loc_n.take() { Some(__sh_loc) => { __SHV_n.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
    return __sh_value;
}