use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_outer_var: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn main() {
    outer_func();
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}

fn outer_func() {
    __SHV_outer_var.with(|v| *v.borrow_mut() = "outer".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    inner_func();
}

fn inner_func() {
    let inner_var = "inner".to_string();
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("Outer: {}, Inner: {}", __SHV_outer_var.with(|v| v.borrow().clone()), inner_var.clone()));
    let result = __SHV_outer_var.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + inner_var.clone().trim().parse::<i64>().unwrap_or(0);
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("Result: {}", result));
    __SH_RC.store(0, Ordering::SeqCst);
}