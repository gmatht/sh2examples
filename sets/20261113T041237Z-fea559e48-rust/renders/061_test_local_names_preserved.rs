use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_new_value: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn main() {
    println!("{}", test_math("10".to_string().to_string(), "5".to_string().to_string(), "add".to_string().to_string()));
    println!("{}", test_math("10".to_string().to_string(), "5".to_string().to_string(), "multiply".to_string().to_string()));
    println!("{}", test_strings("hello world".to_string().to_string(), "start".to_string().to_string(), "hi".to_string().to_string()));
    println!("{}", test_strings("hello world".to_string().to_string(), "end".to_string().to_string(), "bye".to_string().to_string()));
    println!("{}", test_arrays("my_array".to_string().to_string(), "first".to_string().to_string(), "new_value".to_string().to_string()));
    println!("{}", test_arrays("my_array".to_string().to_string(), "last".to_string().to_string(), "final_value".to_string().to_string()));
}

fn test_math(p0: String, p1: String, p2: String) -> String {
    let first_number = format!("{}", p0.clone());
    __SH_RC.store(0, Ordering::SeqCst);
    let second_number = format!("{}", p1.clone());
    __SH_RC.store(0, Ordering::SeqCst);
    let operation = format!("{}", p2.clone());
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_case3 = operation.clone();
    if (__sh_case3 == "add") {
        let __v = { let __v = ((first_number.clone().trim().parse::<i64>().unwrap_or(0) + second_number.clone().trim().parse::<i64>().unwrap_or(0))).to_string(); if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { String::new() } else { __v } }; *__SH_RET.lock().unwrap() = __v.clone(); return __v;
    } else if ((__sh_case3 == "subtract")) {
        let __v = { let __v = ((first_number.clone().trim().parse::<i64>().unwrap_or(0) - second_number.clone().trim().parse::<i64>().unwrap_or(0))).to_string(); if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { String::new() } else { __v } }; *__SH_RET.lock().unwrap() = __v.clone(); return __v;
    } else if ((__sh_case3 == "multiply")) {
        let __v = { let __v = ((first_number.clone().trim().parse::<i64>().unwrap_or(0) * second_number.clone().trim().parse::<i64>().unwrap_or(0))).to_string(); if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { String::new() } else { __v } }; *__SH_RET.lock().unwrap() = __v.clone(); return __v;
    } else if true {
        let __v = format!("Unknown operation: {}", operation.clone()); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    __SH_RET.lock().unwrap().clone()
}

fn test_strings(p0: String, p1: String, p2: String) -> String {
    let input_string = format!("{}", p0.clone());
    __SH_RC.store(0, Ordering::SeqCst);
    let search_pattern = format!("{}", p1.clone());
    __SH_RC.store(0, Ordering::SeqCst);
    let replacement = format!("{}", p2.clone());
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_case4 = search_pattern.clone();
    if (__sh_case4 == "start") {
        let __v = format!("Replacing start of: {} with: {}", input_string.clone(), replacement.clone()); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
    } else if ((__sh_case4 == "end")) {
        let __v = format!("Replacing end of: {} with: {}", input_string.clone(), replacement.clone()); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
    } else if ((__sh_case4 == "middle")) {
        let __v = format!("Replacing middle of: {} with: {}", input_string.clone(), replacement.clone()); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
    } else if true {
        let __v = format!("Unknown pattern: {} for string: {}", search_pattern.clone(), input_string.clone()); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    __SH_RET.lock().unwrap().clone()
}

fn test_arrays(p0: String, p1: String, p2: String) -> String {
    let array_name = format!("{}", p0.clone());
    __SH_RC.store(0, Ordering::SeqCst);
    let index = format!("{}", p1.clone());
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_new_value.with(|v| *v.borrow_mut() = format!("{}", p2.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_case5 = index.clone();
    if (__sh_case5 == "first") {
        let __v = format!("Setting first element of {} to {}", array_name.clone(), __SHV_new_value.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
    } else if ((__sh_case5 == "last")) {
        let __v = format!("Setting last element of {} to {}", array_name.clone(), __SHV_new_value.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
    } else if true {
        let __v = format!("Setting element {} of {} to {}", index.clone(), array_name.clone(), __SHV_new_value.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    __SH_RET.lock().unwrap().clone()
}