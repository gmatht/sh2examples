use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_new_value: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
}

fn main() -> std::process::ExitCode {
    println!("{}", test_math("10".to_string(), "5".to_string(), "add".to_string()));
    println!("{}", test_math("10".to_string(), "5".to_string(), "multiply".to_string()));
    println!("{}", test_strings("hello world".to_string(), "start".to_string(), "hi".to_string()));
    println!("{}", test_strings("hello world".to_string(), "end".to_string(), "bye".to_string()));
    println!("{}", test_arrays("my_array".to_string(), "first".to_string(), "new_value".to_string()));
    println!("{}", test_arrays("my_array".to_string(), "last".to_string(), "final_value".to_string()));
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn test_math(p0: String, p1: String, p2: String) -> String {
    let first_number = format!("{}", p0.clone());
    let second_number = format!("{}", p1.clone());
    let operation = format!("{}", p2.clone());
    if operation.clone() == "add".to_string() {
        let __sh_value = { let __sh_value = ((first_number.trim().parse::<i64>().unwrap_or(0) + second_number.trim().parse::<i64>().unwrap_or(0))).to_string(); if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { String::new() } else { __sh_value } }; *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        return __sh_value;
    } else if operation.clone() == "subtract".to_string() {
        let __sh_value = { let __sh_value = ((first_number.trim().parse::<i64>().unwrap_or(0) - second_number.trim().parse::<i64>().unwrap_or(0))).to_string(); if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { String::new() } else { __sh_value } }; *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        return __sh_value;
    } else if operation.clone() == "multiply".to_string() {
        let __sh_value = { let __sh_value = ((first_number.trim().parse::<i64>().unwrap_or(0) * second_number.trim().parse::<i64>().unwrap_or(0))).to_string(); if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { String::new() } else { __sh_value } }; *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        return __sh_value;
    } else {
        let __sh_value = format!("Unknown operation: {}", operation.clone()); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        return __sh_value;
    }
}

#[allow(dead_code)]
fn test_strings(p0: String, p1: String, p2: String) -> String {
    let input_string = format!("{}", p0.clone());
    let search_pattern = format!("{}", p1.clone());
    let replacement = format!("{}", p2.clone());
    if search_pattern.clone() == "start".to_string() {
        let __sh_value = format!("Replacing start of: {} with: {}", input_string.clone(), replacement.clone()); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        return __sh_value;
    } else if search_pattern.clone() == "end".to_string() {
        let __sh_value = format!("Replacing end of: {} with: {}", input_string.clone(), replacement.clone()); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        return __sh_value;
    } else if search_pattern.clone() == "middle".to_string() {
        let __sh_value = format!("Replacing middle of: {} with: {}", input_string.clone(), replacement.clone()); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        return __sh_value;
    } else {
        let __sh_value = format!("Unknown pattern: {} for string: {}", search_pattern.clone(), input_string.clone()); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        return __sh_value;
    }
}

#[allow(dead_code)]
fn test_arrays(p0: String, p1: String, p2: String) -> String {
    let __sh_loc_new_value: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
    let array_name = format!("{}", p0.clone());
    let index = format!("{}", p1.clone());
    __SHV_new_value.with(|v| *v.borrow_mut() = format!("{}", p2.clone()));
    if index.clone() == "first".to_string() {
        let __sh_value = format!("Setting first element of {} to {}", array_name.clone(), __SHV_new_value.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        match __sh_loc_new_value.take() { Some(__sh_loc) => { __SHV_new_value.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
        return __sh_value;
    } else if index.clone() == "last".to_string() {
        let __sh_value = format!("Setting last element of {} to {}", array_name.clone(), __SHV_new_value.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        match __sh_loc_new_value.take() { Some(__sh_loc) => { __SHV_new_value.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
        return __sh_value;
    } else {
        let __sh_value = format!("Setting element {} of {} to {}", index.clone(), array_name.clone(), __SHV_new_value.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed);
        match __sh_loc_new_value.take() { Some(__sh_loc) => { __SHV_new_value.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
        return __sh_value;
    }
    match __sh_loc_new_value.take() { Some(__sh_loc) => { __SHV_new_value.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
}