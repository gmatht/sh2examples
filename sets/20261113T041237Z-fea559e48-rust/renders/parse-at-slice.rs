static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    let __new = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()]; *__SH_ARGV.lock().unwrap() = __new;
    println!("{}", { let __v = __SH_ARGV.lock().unwrap().clone(); let __s = 2usize.min(__v.len()); __v[__s..].to_vec() }.join(" "));
}