fn main() -> std::process::ExitCode {
    let filesystem_options = ["a".to_string(), "b".to_string(), "c".to_string()];
    println!("{}", filesystem_options[1].clone());
    std::process::ExitCode::SUCCESS
}