static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    let __sh_case0 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() };
    if (__sh_case0 == "on") {
        println!("{} {} {} {}", "Please".to_string(), "rerun".to_string(), format!("\\`{}", std::env::args().next().unwrap_or_default()), "on'".to_string());
    } else {
    }
}