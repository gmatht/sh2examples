static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_a: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", "== Argument count ==".to_string());
    println!("{}", __SH_ARGV.lock().unwrap().len().to_string());
    println!("{}", "== Arguments ==".to_string());
    let __sh_items0 = [__SH_ARGV.lock().unwrap().clone()].concat();
    let mut __sh_i1: usize = 0;
    loop {
    if !(__sh_i1 < __sh_items0.len()) { break; }
        __SHV_a.with(|v| *v.borrow_mut() = __sh_items0[__sh_i1].clone());
        println!("{}", format!("Arg: {}", __SHV_a.with(|v| v.borrow().clone())));
    __sh_i1 += 1;
    }
}