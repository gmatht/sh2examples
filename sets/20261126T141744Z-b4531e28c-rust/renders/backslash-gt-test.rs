fn main() -> std::process::ExitCode {
    if "5".to_string() > "10".to_string() {
        print!("a > b\n");
    } else {
    }
    print!("a=[5]\n");
    print!("b=[10]\n");
    std::process::ExitCode::SUCCESS
}