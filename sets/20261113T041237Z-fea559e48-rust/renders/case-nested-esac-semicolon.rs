static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    let __sh_case0 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() };
    if (__sh_case0 == "a") {
        let __sh_case1 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(1).cloned().unwrap_or_default() };
        if (__sh_case1 == "x") {
            print!("x\n");
        } else {
        }
    } else if ((__sh_case0 == "b")) {
        print!("b\n");
    } else {
    }
}