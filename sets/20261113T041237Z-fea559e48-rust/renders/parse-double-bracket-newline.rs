thread_local! { #[allow(non_upper_case_globals)] static __SHV_other: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_var: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    if (__SHV_var.with(|v| v.borrow().clone()) != "".to_string()) && (!(__SHV_other.with(|v| v.borrow().clone()) != "".to_string())) {
        print!("test\n");
    } else {
    }
}