use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_r: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_x: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    if !{ let _ = { let __v = "hello".to_string(); __SHV_r.with(|v| *v.borrow_mut() = __v.clone()); __v }; __SH_RC.load(Ordering::SeqCst) == 0 } {
        let _ = { let _ = {
__SHV_x.with(|v| *v.borrow_mut() = __SH_RC.load(Ordering::SeqCst).to_string());
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
}; String::new() };
        __SH_RC.store(0, Ordering::SeqCst);
    }
    println!("{}", __SHV_r.with(|v| v.borrow().clone()));
    print!("{}", format!("{}=[{}]\n", "x".to_string(), __SHV_x.with(|v| v.borrow().clone())));
    __SH_RC.store(0, Ordering::SeqCst);
}