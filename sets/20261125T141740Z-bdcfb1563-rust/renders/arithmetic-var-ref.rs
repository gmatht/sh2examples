fn main() -> std::process::ExitCode {
    let var = String::new();
    println!("{}", (var.trim().parse::<i64>().unwrap_or(0) + 1i64));
    std::process::ExitCode::SUCCESS
}