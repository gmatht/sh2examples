thread_local! { #[allow(non_upper_case_globals)] static __SHV_n: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_n.with(|v| *v.borrow_mut() = "5".to_string());
    if __SHV_n.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) == 5 {
        print!("equal\n");
    } else {
    }
    print!("n=[{}]\n", __SHV_n.with(|v| v.borrow().clone()));
    std::process::ExitCode::SUCCESS
}