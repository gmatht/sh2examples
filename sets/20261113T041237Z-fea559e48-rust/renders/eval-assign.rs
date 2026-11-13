thread_local! { #[allow(non_upper_case_globals)] static __SHV_x: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_y: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_x.with(|v| v.set(5));
    __SHV_y.with(|v| *v.borrow_mut() = format!("{}+1", __SHV_x.with(|v| v.get()).to_string()));
    println!("{}", format!("y={}", __SHV_y.with(|v| v.borrow().clone())));
}