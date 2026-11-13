thread_local! { #[allow(non_upper_case_globals)] static __SHV_var: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", (__SHV_var.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + 1i64));
}