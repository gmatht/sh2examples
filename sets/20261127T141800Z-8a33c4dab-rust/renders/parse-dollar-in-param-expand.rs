fn main() -> std::process::ExitCode {
    let x = format!("{}extra", std::env::var("y").unwrap_or_default());
    println!("{}", x.clone());
    std::process::ExitCode::SUCCESS
}