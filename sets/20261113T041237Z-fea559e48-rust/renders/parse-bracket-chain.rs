thread_local! { #[allow(non_upper_case_globals)] static __SHV_VAR1: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_VAR2: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    if (!(__SHV_VAR1.with(|v| v.borrow().clone()) != "".to_string())) && (!(__SHV_VAR2.with(|v| v.borrow().clone()) != "".to_string())) {
        print!("Both empty\n");
    } else {
    }
}