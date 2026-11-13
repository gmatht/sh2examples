thread_local! { #[allow(non_upper_case_globals)] static __SHV_n: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_n.with(|v| *v.borrow_mut() = "5".to_string());
    if __SHV_n.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) == 5 {
        print!("equal\n");
    }
    print!("{}", format!("{}=[{}]\n", "n".to_string(), __SHV_n.with(|v| v.borrow().clone())));
}