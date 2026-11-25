static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn __sh_arg(i: usize) -> String {
    __SH_ARGV.lock().unwrap().get(i).cloned().unwrap_or_default()
}

fn main() -> std::process::ExitCode {
    print!("Testing simple case patterns...\n");
    if __sh_arg(0) == "test".to_string() {
        print!("Matched test\n");
    } else {
        print!("Default case\n");
    }
    std::process::ExitCode::SUCCESS
}