use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_file: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_files: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

fn main() -> std::process::ExitCode {
    { let __sh_operand: Vec<String> = vec!["file1.txt".to_string(), "file2.txt".to_string(), "nonexistent.txt".to_string()]; process_files(&__sh_operand); }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn process_files(__argv: &[String]) {
    __SHV_files.with(|v| *v.borrow_mut() = __argv.to_vec());
    let mut count = 0;
    let __sh_items4 = { let __sh_value = __SHV_files.with(|v| v.borrow().clone()); let __sh_offset = if 0 < 0 { (__sh_value.len() as i64 + 0).max(0) } else { 0 } as usize; let __sh_offset = __sh_offset.min(__sh_value.len()); let __sh_len = if -1 < 0 { __sh_value.len() as i64 - __sh_offset as i64 } else { -1 };                          let __sh_start = __sh_offset.max(0) as usize; let __sh_stop = ((__sh_start as i64 + __sh_len).max(__sh_start as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_start);                          __sh_value[__sh_start..__sh_stop].to_vec() };
    let mut __sh_index5: usize = 0;
    loop {
    if !(__sh_index5 < __sh_items4.len()) { break; }
        __SHV_file.with(|v| *v.borrow_mut() = __sh_items4[__sh_index5].clone());
        if std::path::Path::new(&__SHV_file.with(|v| v.borrow().clone())).is_file() {
            let __sh_arith_value6 = { let __sh_out = count; count = count +1; __sh_out };
            let __sh_num7 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value6 };
            __SH_RC.store(if __sh_num7 != 0 { 0 } else { 1 }, Ordering::Relaxed);
            println!("Processing: {}", __SHV_file.with(|v| v.borrow().clone()));
        } else {
        }
    __sh_index5 += 1;
    }
    println!("Total files processed: {}", count);
}