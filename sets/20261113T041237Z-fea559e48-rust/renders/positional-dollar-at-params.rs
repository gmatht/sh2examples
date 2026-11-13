use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_a: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn main() {
    { let __a: Vec<String> = vec!["one".to_string(), "two three".to_string(), "four".to_string()]; show(&__a); }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}

fn show(__argv: &[String]) {
    println!("{}", format!("count={}", __argv.len().to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("all={}", __argv.to_vec().join(" ")));
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_items2 = [__argv.to_vec()].concat();
    let mut __sh_i3: usize = 0;
    loop {
    if !(__sh_i3 < __sh_items2.len()) { break; }
        __SHV_a.with(|v| *v.borrow_mut() = __sh_items2[__sh_i3].clone());
        println!("{}", format!("arg={}", __SHV_a.with(|v| v.borrow().clone())));
        __SH_RC.store(0, Ordering::SeqCst);
    __sh_i3 += 1;
    }
}