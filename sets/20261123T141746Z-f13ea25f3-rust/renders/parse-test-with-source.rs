thread_local! { #[allow(non_upper_case_globals)] static __SHV_var: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_var.with(|v| *v.borrow_mut() = "hello".to_string());
    if "source".to_string() == __SHV_var.with(|v| v.borrow().clone()) {
        print!("source matched\n");
    }
    { let __sh_source = std::fs::read_to_string("/etc/profile").unwrap_or_default(); for __line in __sh_source.lines() { if let Some((__sh_num, __sh_piece)) = __line.trim().split_once('=') { if __sh_num == "var" { let __sh_value = __sh_piece.trim_matches(|c| c == '\'' || c == '\"'); __SHV_var.with(|v| *v.borrow_mut() = __sh_value.trim().to_string()); } else { std::env::set_var(__sh_num, __sh_piece.trim()); } } } }
    print!("var=[{}]\n", __SHV_var.with(|v| v.borrow().clone()));
    std::process::ExitCode::SUCCESS
}