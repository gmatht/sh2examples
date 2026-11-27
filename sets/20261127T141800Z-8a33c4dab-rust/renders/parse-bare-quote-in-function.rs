use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_RESULT: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    print!("RESULT=[{}]\n", __SHV_RESULT.with(|v| v.borrow().clone()));
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn test() {
    print!("hello world\n");
    __SHV_RESULT.with(|v| *v.borrow_mut() = "test".to_string());
}