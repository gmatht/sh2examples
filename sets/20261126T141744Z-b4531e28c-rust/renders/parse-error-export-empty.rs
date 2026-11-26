fn main() -> std::process::ExitCode {
    #[allow(non_snake_case)] let ENV = "".to_string();
    std::env::set_var("ENV", "");
    print!("ENV=[{}]\n", ENV.clone());
    std::process::ExitCode::SUCCESS
}