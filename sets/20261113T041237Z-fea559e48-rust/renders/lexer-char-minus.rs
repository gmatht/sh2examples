use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_LANG: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_LANG.with(|v| *v.borrow_mut() = "C".to_string());
    std::env::set_var("LANG", "C");
    let _ = { let __b = "a" == "a"; __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b };
    print!("{}", "parsed OK\n".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("done: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}