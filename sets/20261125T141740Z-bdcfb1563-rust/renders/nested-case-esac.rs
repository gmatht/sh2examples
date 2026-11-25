static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn __sh_arg(i: usize) -> String {
    __SH_ARGV.lock().unwrap().get(i).cloned().unwrap_or_default()
}

fn main() -> std::process::ExitCode {
    if __sh_arg(0) == "start".to_string() {
        print!("Starting...\n");
        if __sh_arg(1) == "fast".to_string() {
            print!("Fast start\n");
        } else if __sh_arg(1) == "slow".to_string() {
            print!("Slow start\n");
        } else {
        }
    } else if __sh_arg(0) == "stop".to_string() {
        print!("Stopping...\n");
    } else if __sh_arg(0) == "restart".to_string() {
        print!("Restarting...\n");
        if __sh_arg(1) == "soft".to_string() {
            print!("Soft restart\n");
        } else if __sh_arg(1) == "hard".to_string() {
            print!("Hard restart\n");
        } else {
        }
    } else {
    }
    std::process::ExitCode::SUCCESS
}