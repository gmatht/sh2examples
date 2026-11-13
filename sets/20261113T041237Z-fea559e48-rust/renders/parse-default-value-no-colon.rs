thread_local! { #[allow(non_upper_case_globals)] static __SHV_UMASK: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_UMASK.with(|v| *v.borrow_mut() = { let __v = __SHV_UMASK.with(|v| v.borrow().clone()); if __v.is_empty() { "0640".to_string() } else { __v.to_string() } });
    println!("{}", __SHV_UMASK.with(|v| v.borrow().clone()));
}