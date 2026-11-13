static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    #[allow(non_snake_case)] let OPTS = { let __v = __SH_ARGV.lock().unwrap().join(" "); if __v.is_empty() { "default".to_string() } else { __v.to_string() } };
    println!("{}", OPTS);
}