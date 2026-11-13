static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    print!("Testing simple case patterns...\n");
    let __sh_case0 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() };
    if (__sh_case0 == "test") {
        print!("Matched test\n");
    } else if true {
        print!("Default case\n");
    } else {
    }
}