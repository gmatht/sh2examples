fn main() -> std::process::ExitCode {
    let p = "/usr/local/bin/example".to_string();
    println!("{}", { let __sh_str: String = p.clone().to_string(); let __sh_delim: &str = "/"; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } });
    std::process::ExitCode::SUCCESS
}