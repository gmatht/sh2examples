thread_local! { #[allow(non_upper_case_globals)] static __SHV_var: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_var.with(|v| *v.borrow_mut() = "hello".to_string());
    if "source".to_string() == __SHV_var.with(|v| v.borrow().clone()) {
        print!("source matched\n");
    }
    { let __src = std::fs::read_to_string("/etc/profile").unwrap_or_default(); for __line in __src.lines() { if let Some((__n, __val)) = __line.trim().split_once('=') { if __n == "var" { let __v = __val.trim_matches(|c| c == '\'' || c == '\"'); __SHV_var.with(|v| *v.borrow_mut() = __v.trim().to_string()); } else { std::env::set_var(__n, __val.trim()); } } } }
    print!("{}", format!("{}=[{}]\n", "var".to_string(), __SHV_var.with(|v| v.borrow().clone())));
}