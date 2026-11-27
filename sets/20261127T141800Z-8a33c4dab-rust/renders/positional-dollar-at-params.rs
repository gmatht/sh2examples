use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_a: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn main() -> std::process::ExitCode {
    { let __sh_operand: Vec<String> = vec!["one".to_string(), "two three".to_string(), "four".to_string()]; show(&__sh_operand); }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn show(__argv: &[String]) {
    println!("count={}", __argv.len() as i64);
    println!("all={}", __argv.to_vec().join(" "));
    let __sh_items2 = __argv.to_vec();
    let mut __sh_index3: usize = 0;
    loop {
    if !(__sh_index3 < __sh_items2.len()) { break; }
        __SHV_a.with(|v| *v.borrow_mut() = __sh_items2[__sh_index3].clone());
        println!("arg={}", __SHV_a.with(|v| v.borrow().clone()));
    __sh_index3 += 1;
    }
}