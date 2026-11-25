fn main() -> std::process::ExitCode {
    let result = String::new();
    print!("result={}\n", { let __sh_value = result.clone(); if __sh_value.is_empty() { "empty".to_string() } else { __sh_value.to_string() } });
    std::process::ExitCode::SUCCESS
}