fn main() -> std::process::ExitCode {
    #[allow(non_snake_case)] let A = { let __sh_str: String = std::env::var("B").unwrap_or_default().to_string(); let __sh_delim: &str = "."; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } };
    #[allow(non_snake_case)] let C = { let __sh_str: String = std::env::var("D").unwrap_or_default().to_string(); let __sh_delim: &str = "_"; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } };
    println!("{} {}", A.clone(), C.clone());
    std::process::ExitCode::SUCCESS
}