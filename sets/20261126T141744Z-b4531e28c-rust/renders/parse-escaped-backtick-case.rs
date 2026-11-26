static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn __sh_arg(i: usize) -> String {
    __SH_ARGV.lock().unwrap().get(i).cloned().unwrap_or_default()
}

fn main() -> std::process::ExitCode {
    if __sh_arg(0) == "on".to_string() {
        println!("{} {} {} {}", "Please", "rerun", format!("\\`{}", std::env::args().next().unwrap_or_default()), "on'");
    } else {
    }
    std::process::ExitCode::SUCCESS
}