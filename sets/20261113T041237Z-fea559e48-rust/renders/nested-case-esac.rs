static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    let __sh_case0 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() };
    if (__sh_case0 == "start") {
        print!("Starting...\n");
        let __sh_case1 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(1).cloned().unwrap_or_default() };
        if (__sh_case1 == "fast") {
            print!("Fast start\n");
        } else if ((__sh_case1 == "slow")) {
            print!("Slow start\n");
        } else {
        }
    } else if ((__sh_case0 == "stop")) {
        print!("Stopping...\n");
    } else if ((__sh_case0 == "restart")) {
        print!("Restarting...\n");
        let __sh_case2 = { let __g = __SH_ARGV.lock().unwrap(); __g.get(1).cloned().unwrap_or_default() };
        if (__sh_case2 == "soft") {
            print!("Soft restart\n");
        } else if ((__sh_case2 == "hard")) {
            print!("Hard restart\n");
        } else {
        }
    } else {
    }
}