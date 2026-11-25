fn main() -> std::process::ExitCode {
    if std::env::var("x").unwrap_or_default() == "a".to_string() {
    } else {
    }
    print!("parsed OK\n");
    std::process::ExitCode::SUCCESS
}