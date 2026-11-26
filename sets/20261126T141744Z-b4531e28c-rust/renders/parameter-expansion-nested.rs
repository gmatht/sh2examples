fn main() -> std::process::ExitCode {
    { let __sh_w = [{ let __sh_split_s: String = { let __sh_value = std::env::var("MOUNTPOINT").unwrap_or_default(); if __sh_value.is_empty() { format!("{}", std::env::var("NAME").unwrap_or_default()) } else { __sh_value.to_string() } }; __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    std::process::ExitCode::SUCCESS
}