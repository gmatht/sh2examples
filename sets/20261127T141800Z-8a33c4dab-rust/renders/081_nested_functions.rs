use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_msg: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn main() -> std::process::ExitCode {
    outer();
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn outer() {
    let __sh_loc_msg: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
    __SHV_msg.with(|v| *v.borrow_mut() = "hello".to_string());
    inner();
    match __sh_loc_msg.take() { Some(__sh_loc) => { __SHV_msg.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
}

#[allow(dead_code)]
fn inner() {
    println!("{}", __SHV_msg.with(|v| v.borrow().clone()));
}