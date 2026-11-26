fn main() -> std::process::ExitCode {
    let s = "abcdef".to_string();
    println!("len={}", (s.clone().len() as i64));
    std::process::ExitCode::SUCCESS
}