fn main() -> std::process::ExitCode {
    print!("Hello, World!\n");
    if std::path::Path::new("test.txt").is_file() {
        print!("File exists\n");
    }
    for _i in [1, 2, 3, 4, 5] {
        { let __sh_w = [{ let __sh_split_s: String = _i.to_string(); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    }
    std::process::ExitCode::SUCCESS
}