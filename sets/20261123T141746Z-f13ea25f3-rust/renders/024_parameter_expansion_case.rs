fn main() -> std::process::ExitCode {
    print!("== Case modification in parameter expansion ==\n");
    let name = "world".to_string();
    println!("{}", name.clone().to_uppercase());
    println!("{}", name.clone().to_lowercase());
    print!("World\n");
    std::process::ExitCode::SUCCESS
}