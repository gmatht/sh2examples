fn main() -> std::process::ExitCode {
    println!("{}", { let __sh_value = std::env::var("MYVAR").unwrap_or_default(); if __sh_value.is_empty() { "default value".to_string() } else { __sh_value.to_string() } });
    std::process::ExitCode::SUCCESS
}