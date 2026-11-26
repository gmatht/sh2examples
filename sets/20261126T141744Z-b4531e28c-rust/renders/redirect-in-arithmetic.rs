fn main() -> std::process::ExitCode {
    let arr = [10, 20, 30];
    println!("{}", ((arr[1] > 2) as i64));
    std::process::ExitCode::SUCCESS
}