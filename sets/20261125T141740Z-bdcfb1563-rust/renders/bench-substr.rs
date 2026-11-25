fn main() -> std::process::ExitCode {
    let s = "abcdefghi".to_string();
    println!("sub={}", s.clone().chars().skip((2) as usize).take((3) as usize).collect::<String>());
    std::process::ExitCode::SUCCESS
}