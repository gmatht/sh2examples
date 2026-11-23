static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() -> std::process::ExitCode {
    let opts = { let __sh_value = __SH_ARGV.lock().unwrap().join(" "); if __sh_value.is_empty() { "--help".to_string() } else { __sh_value.to_string() } };
    println!("{}", opts.clone());
    std::process::ExitCode::SUCCESS
}