thread_local! { #[allow(non_upper_case_globals)] static __SHV_result: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_result.with(|v| *v.borrow_mut() = format!("{}", (std::env::var("var").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) + 0).to_string()));
    println!("{}", format!("Eval result: {}", __SHV_result.with(|v| v.borrow().clone())));
}