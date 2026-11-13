static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_dryrun: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    let __sh_case0 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() };
    if (__sh_case0 == "--dryrun") {
        __SHV_dryrun.with(|v| *v.borrow_mut() = "t".to_string());
        { let mut __v = __SH_ARGV.lock().unwrap(); let __n = (1).min(__v.len() as i64) as usize; __v.drain(0..__n); }
    } else if ((__sh_case0 == "a")) {
        let __new = vec!["hello".to_string()]; *__SH_ARGV.lock().unwrap() = __new;
    } else if ((__sh_case0 == "b")) {
        __SHV_dryrun.with(|v| *v.borrow_mut() = "t".to_string());
    } else {
    }
    print!("{}", "parsed OK\n".to_string());
}