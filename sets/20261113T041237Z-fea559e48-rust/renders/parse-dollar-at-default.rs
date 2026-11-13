static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    let opts = { let __v = __SH_ARGV.lock().unwrap().join(" "); if __v.is_empty() { "--help".to_string() } else { __v.to_string() } };
    println!("{}", opts);
}