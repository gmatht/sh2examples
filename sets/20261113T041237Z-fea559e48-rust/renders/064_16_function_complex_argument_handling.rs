use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_count: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_file: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_files: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

fn main() {
    { let __a: Vec<String> = vec!["file1.txt".to_string(), "file2.txt".to_string(), "nonexistent.txt".to_string()]; process_files(&__a); }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}

fn process_files(__argv: &[String]) {
    __SHV_files.with(|v| *v.borrow_mut() = [__argv.to_vec()].concat());
    __SHV_count.with(|v| v.set(0));
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_items4 = [__SHV_files.with(|v| v.borrow().clone())].concat();
    let mut __sh_i5: usize = 0;
    loop {
    if !(__sh_i5 < __sh_items4.len()) { break; }
        __SHV_file.with(|v| *v.borrow_mut() = __sh_items4[__sh_i5].clone());
        if std::path::Path::new(&__SHV_file.with(|v| v.borrow().clone())).is_file() {
            let __sh_arith_v6 = { let __o = __SHV_count.with(|v| v.get()); __SHV_count.with(|v| v.set(__SHV_count.with(|v| v.get()) +1)); __o };
            let __n7 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v6 };
            __SH_RC.store(if __n7 != 0 { 0 } else { 1 }, Ordering::SeqCst);
            println!("{}", format!("Processing: {}", __SHV_file.with(|v| v.borrow().clone())));
            __SH_RC.store(0, Ordering::SeqCst);
        } else {
            __SH_RC.store(0, Ordering::SeqCst);
        }
    __sh_i5 += 1;
    }
    println!("{}", format!("Total files processed: {}", __SHV_count.with(|v| v.get())));
    __SH_RC.store(0, Ordering::SeqCst);
}