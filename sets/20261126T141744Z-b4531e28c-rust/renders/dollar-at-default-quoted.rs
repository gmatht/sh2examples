static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() -> std::process::ExitCode {
    #[allow(non_snake_case)] let ARGS = { let __sh_value = __SH_ARGV.lock().unwrap().join(" "); if __sh_value.is_empty() { "default".to_string() } else { __sh_value.to_string() } };
    println!("{}", ARGS.clone());
    std::process::ExitCode::SUCCESS
}