thread_local! { #[allow(non_upper_case_globals)] static __SHV_cpu: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    let __sh_case0 = std::env::var("arch").unwrap_or_default();
    if { let __t: &str = {&__sh_case0}; __t.starts_with("i") && __t.ends_with("86") && __t.chars().count() >= 3 } || (__sh_case0 == "x86_64") {
        __SHV_cpu.with(|v| *v.borrow_mut() = std::env::var("basic_machine").unwrap_or_default());
    } else {
    }
    print!("{}", format!("{}=[{}]\n", "cpu".to_string(), __SHV_cpu.with(|v| v.borrow().clone())));
}