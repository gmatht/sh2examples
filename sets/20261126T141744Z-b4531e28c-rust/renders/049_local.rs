thread_local! { #[allow(non_upper_case_globals)] static __SHV_a: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    let mut __sv0_0 = String::new();
    __SHV_a.with(|v| *v.borrow_mut() = "1".to_string());
    { let __sh_w = [{ let __sh_split_s: String = __SHV_a.with(|v| v.borrow().clone()); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    __sv0_0 = __SHV_a.with(|v| v.borrow().clone());
    __SHV_a.with(|v| *v.borrow_mut() = "2".to_string());
    { let __sh_w = [{ let __sh_split_s: String = __SHV_a.with(|v| v.borrow().clone()); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    __SHV_a.with(|v| *v.borrow_mut() = __sv0_0.clone());
    { let __sh_w = [{ let __sh_split_s: String = __SHV_a.with(|v| v.borrow().clone()); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    { let __sh_w = [{ let __sh_split_s: String = __SHV_a.with(|v| v.borrow().clone()); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    std::process::ExitCode::SUCCESS
}