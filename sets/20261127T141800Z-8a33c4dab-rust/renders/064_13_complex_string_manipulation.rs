fn main() -> std::process::ExitCode {
    let filename = "my_file.txt".to_string();
    let basename = { let __sh_str: String = filename.clone().to_string(); let __sh_delim: &str = "."; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } };
    let extension = { let __sh_str: String = filename.clone().to_string(); let __sh_delim: &str = "."; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } };
    let uppercase = filename.clone().to_uppercase();
    let lowercase = filename.clone().to_lowercase();
    println!("Basename: {}", basename.clone());
    println!("Extension: {}", extension.clone());
    println!("Uppercase: {}", uppercase.clone());
    println!("Lowercase: {}", lowercase.clone());
    std::process::ExitCode::SUCCESS
}