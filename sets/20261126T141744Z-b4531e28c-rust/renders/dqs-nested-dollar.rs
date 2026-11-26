fn main() -> std::process::ExitCode {
    let mut name = String::new();
    name = { let __sh_from_set: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); let __sh_to_set: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect(); std::env::var("USER").unwrap_or_default().chars().map(|__sh_ch| match __sh_from_set.iter().position(|&__sh_from_ch| __sh_from_ch == __sh_ch) { Some(__sh_begin) => __sh_to_set[__sh_begin.min(__sh_to_set.len() - 1)], None => __sh_ch }).collect::<String>() };
    println!("{}", name.clone());
    std::process::ExitCode::SUCCESS
}