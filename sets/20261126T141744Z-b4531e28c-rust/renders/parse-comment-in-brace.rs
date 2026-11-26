fn main() -> std::process::ExitCode {
    let mut tmp = String::new();
    if std::env::var("opt").unwrap_or_default() == "".to_string() {
    } else if std::env::var("opt").unwrap_or_default().starts_with("configfile=") {
        tmp = { let __sh_str: String = std::env::var("opt").unwrap_or_default().to_string(); let __sh_delim: &str = "="; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } };
    } else {
        print!("default\n");
    }
    print!("tmp=[{}]\n", tmp.clone());
    print!("configfile=[{}]\n", std::env::var("configfile").unwrap_or_default());
    std::process::ExitCode::SUCCESS
}