fn main() -> std::process::ExitCode {
    #[allow(non_snake_case)] let MYVAR = { let __sh_value = std::env::var("UMASK").unwrap_or_default(); if __sh_value.is_empty() { "0640".to_string() } else { __sh_value.to_string() } };
    println!("{}", MYVAR.clone());
    std::process::ExitCode::SUCCESS
}