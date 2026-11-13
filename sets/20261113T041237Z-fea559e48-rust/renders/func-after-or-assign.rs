static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_VAR: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    if !(__SHV_VAR.with(|v| v.borrow().clone()) != "".to_string()) {
        let _ = { let __v = "default".to_string(); __SHV_VAR.with(|v| *v.borrow_mut() = __v.clone()); __v };
    }
    println!("{}", my_func());
}

fn my_func() -> String {
    let __v = format!("hello {}", __SHV_VAR.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
}