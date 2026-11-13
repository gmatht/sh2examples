static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    let __sh_case0 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() };
    if (__sh_case0 == "start") {
        { let mut __v = __SH_ARGV.lock().unwrap(); let __n = (1).min(__v.len() as i64) as usize; __v.drain(0..__n); }
    } else if ((__sh_case0 == "stop")) {
        let __new = vec!["stopped".to_string()]; *__SH_ARGV.lock().unwrap() = __new;
    } else if ((__sh_case0 == "check")) {
        print!("checking\n");
    } else if true {
        print!("unknown\n");
    } else {
    }
}