fn main() -> std::process::ExitCode {
    println!("{}", { let __sh_str: String = std::env::var("HOME").unwrap_or_default().to_string(); if let Some(__sh_res) = __sh_str.strip_suffix("/") { __sh_res.to_string() } else { __sh_str } });
    std::process::ExitCode::SUCCESS
}