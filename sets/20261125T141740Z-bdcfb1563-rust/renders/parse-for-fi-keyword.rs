fn main() -> std::process::ExitCode {
    for _lang in ["cs", "de", "es", "fi", "fr"] {
        println!("{}", _lang.to_string());
    }
    std::process::ExitCode::SUCCESS
}