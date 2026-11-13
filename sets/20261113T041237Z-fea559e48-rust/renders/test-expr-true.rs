thread_local! { #[allow(non_upper_case_globals)] static __SHV_var: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    if !(__SHV_var.with(|v| v.borrow().clone()) != "true".to_string()) {
        print!("it's true\n");
    } else {
    }
}