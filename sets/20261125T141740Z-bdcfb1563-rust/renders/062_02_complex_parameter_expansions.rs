fn main() -> std::process::ExitCode {
    print!("Testing complex parameter expansions...\n");
    let complex_var = "hello world".to_string();
    println!("{}", { let __sh_str: String = complex_var.clone().to_string(); let __sh_delim: &str = "o"; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } });
    println!("{}", { let __sh_str: String = complex_var.clone().to_string(); let __sh_delim: &str = "o"; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } });
    println!("{}", { let __sh_str: String = complex_var.clone().to_string(); let __sh_delim: &str = "o"; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } });
    println!("{}", { let __sh_str: String = complex_var.clone().to_string(); let __sh_delim: &str = "o"; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } });
    std::process::ExitCode::SUCCESS
}