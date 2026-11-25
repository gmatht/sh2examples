fn main() -> std::process::ExitCode {
    let x = { let __sh_value = std::env::var("foo").unwrap_or_default(); if __sh_value.is_empty() { "bar\nbaz".to_string() } else { __sh_value.to_string() } };
    println!("{}", x.clone());
    std::process::ExitCode::SUCCESS
}