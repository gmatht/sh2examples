thread_local! { #[allow(non_upper_case_globals)] static __SHV_maybe: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    print!("== Default values ==\n");
    __SHV_maybe.with(|v| *v.borrow_mut() = String::new());
    println!("{}", { let __sh_value = __SHV_maybe.with(|v| v.borrow().clone()); if __sh_value.is_empty() { "default".to_string() } else { __sh_value.to_string() } });
    println!("{}", { let __sh_value = __SHV_maybe.with(|v| v.borrow().clone()); if __sh_value.is_empty() { let __sh_default = "default".to_string(); __SHV_maybe.with(|v| *v.borrow_mut() = __sh_default.clone()); __sh_default.clone() } else { __sh_value } });
    println!("{}", { let __sh_value = __SHV_maybe.with(|v| v.borrow().clone()); if __sh_value.is_empty() { eprintln!("sh2: {}: {}", "maybe", "error"); std::process::exit(1); } __sh_value });
    std::process::ExitCode::SUCCESS
}