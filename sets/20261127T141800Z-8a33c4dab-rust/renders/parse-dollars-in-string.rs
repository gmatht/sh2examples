static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    let __sh_items0 = __SH_ARGV.lock().unwrap().clone();
    let mut __sh_index1: usize = 0;
    loop {
    if !(__sh_index1 < __sh_items0.len()) { break; }
        __SHV_i.with(|v| *v.borrow_mut() = __sh_items0[__sh_index1].clone());
        println!("{}", __SHV_i.with(|v| v.borrow().clone()));
    __sh_index1 += 1;
    }
    let __sh_items2 = vec![__SH_ARGV.lock().unwrap().join(" ")];
    let mut __sh_index3: usize = 0;
    loop {
    if !(__sh_index3 < __sh_items2.len()) { break; }
        __SHV_i.with(|v| *v.borrow_mut() = __sh_items2[__sh_index3].clone());
        println!("{}", __SHV_i.with(|v| v.borrow().clone()));
    __sh_index3 += 1;
    }
    std::process::ExitCode::SUCCESS
}