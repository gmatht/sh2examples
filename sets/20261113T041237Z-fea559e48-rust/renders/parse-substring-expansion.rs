static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    let __new = vec![{ let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() }, { let __g = __SH_ARGV.lock().unwrap(); __g.get(1).cloned().unwrap_or_default() }, { let __g = __SH_ARGV.lock().unwrap(); __g.get(2).cloned().unwrap_or_default() }]; *__SH_ARGV.lock().unwrap() = __new;
    println!("{}", { let __v = __SH_ARGV.lock().unwrap().clone(); let __s = 2usize.min(__v.len()); __v[__s..].to_vec() }.join(" "));
}