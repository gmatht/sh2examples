fn main() -> std::process::ExitCode {
    if std::env::var("needop").unwrap_or_default() == "".to_string() {
        print!("empty\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}