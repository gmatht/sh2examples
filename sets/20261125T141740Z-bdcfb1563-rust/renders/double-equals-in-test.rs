fn main() -> std::process::ExitCode {
    let var = String::new();
    if var.clone() == "value".to_string() {
        print!("match\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}