thread_local! { #[allow(non_upper_case_globals)] static __SHV_array: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    print!("Testing complex array operations...\n");
    __SHV_array.with(|v| *v.borrow_mut() = vec!["item1".to_string(), "item2".to_string(), "item3".to_string()]);
    __SHV_array.with(|v| { let mut b = v.borrow_mut(); for __sh_word in vec!["item4".to_string()] { b.push(__sh_word); } });
    println!("Array: {}", __SHV_array.with(|v| v.borrow().clone()).join(" "));
    println!("Length: {}", __SHV_array.with(|v| v.borrow().len() as i64).to_string());
    println!("First item: {}", { let __sh_idx = 0; let __sh_num = __SHV_array.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_array.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } });
    std::process::ExitCode::SUCCESS
}