fn main() -> std::process::ExitCode {
    let arr = ["one".to_string(), "two".to_string(), "three".to_string()];
    println!("{}", arr[1].clone());
    std::process::ExitCode::SUCCESS
}