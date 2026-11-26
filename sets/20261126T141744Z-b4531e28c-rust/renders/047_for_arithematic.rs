thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_j: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_j.with(|v| *v.borrow_mut() = "1".to_string());
    for __sh_w0 in ["1", "2", "3", "4", "5"] {
        __SHV_i.with(|v| *v.borrow_mut() = __sh_w0.to_string());
        __SHV_j.with(|v| *v.borrow_mut() = (__SHV_j.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) * __SHV_i.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0))).to_string());
    }
    { let __sh_w = [{ let __sh_split_s: String = __SHV_j.with(|v| v.borrow().clone()); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    std::process::ExitCode::SUCCESS
}