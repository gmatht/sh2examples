fn main() -> std::process::ExitCode {
    let other = String::new();
    let var = String::new();
    if (var.clone() != "".to_string()) && (other.clone() == "".to_string()) {
        print!("test\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}