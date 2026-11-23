fn main() -> std::process::ExitCode {
    let s = "abc:def:ghi".to_string();
    println!("short-prefix={}", { let __sh_str: String = s.clone().to_string(); let __sh_delim: &str = ":"; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } });
    println!("long-prefix={}", { let __sh_str: String = s.clone().to_string(); let __sh_delim: &str = ":"; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } });
    println!("short-suffix={}", { let __sh_str: String = s.clone().to_string(); let __sh_delim: &str = ":"; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } });
    println!("long-suffix={}", { let __sh_str: String = s.clone().to_string(); let __sh_delim: &str = ":"; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } });
    std::process::ExitCode::SUCCESS
}