fn main() -> std::process::ExitCode {
    if !{ let __sh_value = std::env::var("MYVAR").unwrap_or_default(); if __sh_value.is_empty() { String::new() } else { "set".to_string() } }.is_empty() {
        print!("MYVAR is set\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}