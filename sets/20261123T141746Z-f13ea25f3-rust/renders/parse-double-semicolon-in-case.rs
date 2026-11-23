static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_dryrun: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_arg(i: usize) -> String {
    __SH_ARGV.lock().unwrap().get(i).cloned().unwrap_or_default()
}

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    if __sh_arg(0) == "--dryrun".to_string() {
        __SHV_dryrun.with(|v| *v.borrow_mut() = "t".to_string());
        { let mut __sh_value = __SH_ARGV.lock().unwrap(); let __sh_num = (1).min(__sh_value.len() as i64) as usize; __sh_value.drain(0..__sh_num); }
    } else if __sh_arg(0) == "a".to_string() {
        let __sh_newargv = vec!["hello".to_string()]; *__SH_ARGV.lock().unwrap() = __sh_newargv;
    } else if __sh_arg(0) == "b".to_string() {
        __SHV_dryrun.with(|v| *v.borrow_mut() = "t".to_string());
    } else {
    }
    print!("parsed OK\n");
    std::process::ExitCode::SUCCESS
}