fn main() -> std::process::ExitCode {
    let x = (std::env::var("n").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) + 1) % 3;
    print!("x=[{}]\n", x.to_string());
    std::process::ExitCode::SUCCESS
}