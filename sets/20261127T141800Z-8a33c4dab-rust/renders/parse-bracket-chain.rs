fn main() -> std::process::ExitCode {
    #[allow(non_snake_case)] let VAR1 = String::new();
    #[allow(non_snake_case)] let VAR2 = String::new();
    if (VAR1.clone() == "".to_string()) && (VAR2.clone() == "".to_string()) {
        print!("Both empty\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}