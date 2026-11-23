fn main() -> std::process::ExitCode {
    let var = String::new();
    if var.clone() == "true".to_string() {
        print!("it's true\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}