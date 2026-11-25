fn main() -> std::process::ExitCode {
    if std::path::Path::new(&std::env::var("file").unwrap_or_default()).is_file() {
        print!("exists\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}