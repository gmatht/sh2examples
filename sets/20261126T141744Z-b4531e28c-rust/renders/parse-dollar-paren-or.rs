use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_x: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    let mut r = String::new();
    { let __sh_buf = { let _ = { let __sh_value = "hello".to_string(); r = "hello".to_string(); __sh_value }; __SH_RC.load(Ordering::Relaxed) == 0 }; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        let _ = { let _ = {
__SHV_x.with(|v| *v.borrow_mut() = __SH_RC.load(Ordering::Relaxed).to_string());
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; String::new() };
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    println!("{}", r.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    print!("x=[{}]\n", __SHV_x.with(|v| v.borrow().clone()));
    __SH_RC.store(0, Ordering::Relaxed);
    std::process::ExitCode::SUCCESS
}