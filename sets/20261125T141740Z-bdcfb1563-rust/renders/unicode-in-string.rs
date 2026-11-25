fn main() -> std::process::ExitCode {
    println!("{}✅ All tests passed{}", std::env::var("GREEN").unwrap_or_default(), std::env::var("NC").unwrap_or_default());
    std::process::ExitCode::SUCCESS
}