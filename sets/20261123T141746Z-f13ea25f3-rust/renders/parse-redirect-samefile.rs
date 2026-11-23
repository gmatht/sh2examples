fn main() -> std::process::ExitCode {
    println!("{}", { let __sh_str: String = std::env::var("PATH").unwrap_or_default().to_string(); let __sh_delim: &str = ":"; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } });
    std::process::ExitCode::SUCCESS
}