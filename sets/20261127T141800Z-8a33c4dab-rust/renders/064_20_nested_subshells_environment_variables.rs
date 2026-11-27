thread_local! { #[allow(non_upper_case_globals)] static __SHV_DEBUG: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_LOG_LEVEL: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_DEBUG.with(|v| *v.borrow_mut() = "1".to_string());
    std::env::set_var("DEBUG", "1");
    __SHV_LOG_LEVEL.with(|v| *v.borrow_mut() = "verbose".to_string());
    std::env::set_var("LOG_LEVEL", "verbose");
    let __sh_sv0 = __SHV_DEBUG.with(|v| v.borrow().clone());
    __SHV_DEBUG.with(|v| *v.borrow_mut() = String::new());
    println!("Inner: LOG_LEVEL={}, DEBUG={}", __SHV_LOG_LEVEL.with(|v| v.borrow().clone()), { let __sh_value = __SHV_DEBUG.with(|v| v.borrow().clone()); if __sh_value.is_empty() { "unset".to_string() } else { __sh_value.to_string() } });
    __SHV_DEBUG.with(|v| *v.borrow_mut() = __sh_sv0);
    println!("Outer: LOG_LEVEL={}, DEBUG={}", __SHV_LOG_LEVEL.with(|v| v.borrow().clone()), __SHV_DEBUG.with(|v| v.borrow().clone()));
    std::process::ExitCode::SUCCESS
}