thread_local! { #[allow(non_upper_case_globals)] static __SHV_x: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    print!("== Indexed arrays ==\n");
    let arr = ["one".to_string(), "two".to_string(), "three".to_string()];
    println!("{}", arr[1].clone());
    println!("{}", 3i64);
    let __sh_items0 = { let __sh_value = arr.to_vec(); let __sh_offset = if 0 < 0 { (__sh_value.len() as i64 + 0).max(0) } else { 0 } as usize; let __sh_offset = __sh_offset.min(__sh_value.len()); let __sh_len = if -1 < 0 { __sh_value.len() as i64 - __sh_offset as i64 } else { -1 };                          let __sh_start = __sh_offset.max(0) as usize; let __sh_stop = ((__sh_start as i64 + __sh_len).max(__sh_start as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_start);                          __sh_value[__sh_start..__sh_stop].to_vec() };
    let mut __sh_index1: usize = 0;
    loop {
    if !(__sh_index1 < __sh_items0.len()) { break; }
        __SHV_x.with(|v| *v.borrow_mut() = __sh_items0[__sh_index1].clone());
        print!("{} ", __SHV_x.with(|v| v.borrow().clone()));
    __sh_index1 += 1;
    }
    println!();
    std::process::ExitCode::SUCCESS
}