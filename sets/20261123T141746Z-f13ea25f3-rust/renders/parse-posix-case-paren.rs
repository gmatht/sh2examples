static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn __sh_arg(i: usize) -> String {
    __SH_ARGV.lock().unwrap().get(i).cloned().unwrap_or_default()
}

fn main() -> std::process::ExitCode {
    if __sh_arg(0) == "start".to_string() {
        print!("starting\n");
    } else if __sh_arg(0) == "stop".to_string() {
        print!("stopping\n");
    } else {
        print!("unknown\n");
    }
    std::process::ExitCode::SUCCESS
}