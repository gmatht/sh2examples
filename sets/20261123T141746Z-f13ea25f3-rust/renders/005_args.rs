static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_a: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    print!("== Argument count ==\n");
    println!("{}", __SH_ARGV.lock().unwrap().len() as i64);
    print!("== Arguments ==\n");
    let __sh_items0 = __SH_ARGV.lock().unwrap().clone();
    let mut __sh_index1: usize = 0;
    loop {
    if !(__sh_index1 < __sh_items0.len()) { break; }
        __SHV_a.with(|v| *v.borrow_mut() = __sh_items0[__sh_index1].clone());
        println!("Arg: {}", __SHV_a.with(|v| v.borrow().clone()));
    __sh_index1 += 1;
    }
    std::process::ExitCode::SUCCESS
}