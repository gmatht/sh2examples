fn main() {
    #[allow(non_snake_case)] let ENV = "".to_string();
    std::env::set_var("ENV", "");
    print!("{}", format!("ENV=[{}]\n", ENV.clone()));
}