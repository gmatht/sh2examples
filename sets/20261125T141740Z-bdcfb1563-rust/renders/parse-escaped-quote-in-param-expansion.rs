fn main() -> std::process::ExitCode {
    let mut val = String::new();
    val = { let __sh_value = val.clone(); __sh_value.strip_prefix("\"").unwrap_or(&__sh_value).to_string() };
    val = { let __sh_value = val.clone(); __sh_value.strip_suffix("\"").unwrap_or(&__sh_value).to_string() };
    println!("{}", val);
    std::process::ExitCode::SUCCESS
}