use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_outer_var: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn main() -> std::process::ExitCode {
    outer_func();
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn outer_func() {
    __SHV_outer_var.with(|v| *v.borrow_mut() = "outer".to_string());
    inner_func();
}

#[allow(dead_code)]
fn inner_func() {
    let inner_var = "inner".to_string();
    println!("Outer: {}, Inner: {}", __SHV_outer_var.with(|v| v.borrow().clone()), inner_var.clone());
    let result = __SHV_outer_var.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) + inner_var.trim().parse::<i64>().unwrap_or(0);
    println!("Result: {}", result);
}