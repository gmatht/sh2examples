fn main() -> std::process::ExitCode {
    print!("== Advanced parameter expansion ==\n");
    let path = "/tmp/025_param_expansion_file.txt".to_string();
    println!("{}", { let __sh_str: String = path.clone().to_string(); let __sh_delim: &str = "/"; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } });
    println!("{}", { let __sh_str: String = path.clone().to_string(); let __sh_delim: &str = "/"; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } });
    print!("aXXa\n");
    std::process::ExitCode::SUCCESS
}