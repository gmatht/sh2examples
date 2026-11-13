thread_local! { #[allow(non_upper_case_globals)] static __SHV_var: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", "1 2 3 4 5".to_string());
    println!("{}", __SHV_var.with(|v| v.borrow().clone()).join(" "));
}