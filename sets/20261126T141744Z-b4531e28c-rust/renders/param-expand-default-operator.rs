fn main() -> std::process::ExitCode {
    if !{ let __sh_value = std::env::var("ZSH_VERSION").unwrap_or_default(); if __sh_value.is_empty() { String::new() } else { __sh_value.to_string() } }.is_empty() {
        print!("zsh\n");
    } else if !{ let __sh_value = "5.1.16".to_string(); if __sh_value.is_empty() { String::new() } else { __sh_value.to_string() } }.is_empty() {
        print!("bash\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}