fn main() -> std::process::ExitCode {
    #[allow(non_snake_case)] let mut UMASK = String::new();
    UMASK = { let __sh_value = UMASK.clone(); if __sh_value.is_empty() { "0640".to_string() } else { __sh_value.to_string() } };
    println!("{}", UMASK.clone());
    std::process::ExitCode::SUCCESS
}