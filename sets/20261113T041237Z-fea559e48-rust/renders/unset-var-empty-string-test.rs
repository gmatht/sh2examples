thread_local! { #[allow(non_upper_case_globals)] static __SHV_never_set2: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    if std::env::var("never_set").unwrap_or_default() == "".to_string() {
        print!("unset-is-empty\n");
    } else {
        print!("unset-not-empty\n");
    }
    if !(__SHV_never_set2.with(|v| v.borrow().clone()) != "".to_string()) {
        print!("zero-len\n");
    } else {
    }
}