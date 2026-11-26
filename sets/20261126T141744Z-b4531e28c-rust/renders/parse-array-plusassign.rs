thread_local! { #[allow(non_upper_case_globals)] static __SHV_LIST: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_LIST.with(|v| *v.borrow_mut() = Vec::<String>::new());
    __SHV_LIST.with(|v| { let mut b = v.borrow_mut(); for __sh_word in vec!["item1".to_string(), "item2".to_string()] { b.push(__sh_word); } });
    print!("LIST=[{}]\n", { let __sh_value = __SHV_LIST.with(|v| v.borrow().get(0 as usize).cloned().unwrap_or_default()); if __sh_value.is_empty() { "".to_string() } else { __sh_value.to_string() } });
    std::process::ExitCode::SUCCESS
}