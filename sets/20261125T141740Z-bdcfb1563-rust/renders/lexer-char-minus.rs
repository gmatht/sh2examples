use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_LANG: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_LANG.with(|v| *v.borrow_mut() = "C".to_string());
    std::env::set_var("LANG", "C");
    { let __sh_buf = "a" == "a"; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    print!("parsed OK\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("done: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}