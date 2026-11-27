fn main() -> std::process::ExitCode {
    let x = { let __sh_str: String = std::env::var("y").unwrap_or_default().to_string(); let __sh_delim: &str = " "; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } };
    println!("{}", x.clone());
    std::process::ExitCode::SUCCESS
}