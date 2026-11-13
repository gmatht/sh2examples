thread_local! { #[allow(non_upper_case_globals)] static __SHV_cpu: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    let mut arch = "x86_64".to_string();
    let __sh_case0 = arch.clone();
    if { let __t: &str = {&__sh_case0}; __t.starts_with("i") && __t.ends_with("86") && __t.chars().count() >= 3 } || (__sh_case0 == "x86_64") {
        __SHV_cpu.with(|v| *v.borrow_mut() = "pc".to_string());
    } else if true {
        __SHV_cpu.with(|v| *v.borrow_mut() = "unknown".to_string());
    } else {
    }
    print!("{}", format!("{}=[{}]\n", "cpu".to_string(), __SHV_cpu.with(|v| v.borrow().clone())));
    arch = "arm64".to_string();
    let __sh_case1 = arch.clone();
    if { let __t: &str = {&__sh_case1}; __t.starts_with("i") && __t.ends_with("86") && __t.chars().count() >= 3 } || (__sh_case1 == "x86_64") {
        __SHV_cpu.with(|v| *v.borrow_mut() = "pc".to_string());
    } else if true {
        __SHV_cpu.with(|v| *v.borrow_mut() = "unknown".to_string());
    } else {
    }
    print!("{}", format!("unknown: {}=[{}]\n", "cpu".to_string(), __SHV_cpu.with(|v| v.borrow().clone())));
}