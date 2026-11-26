fn main() -> std::process::ExitCode {
    let mut name = String::new();
    print!("hi 42\n");
    print!("a\tb\n");
    name = format!("{}-{}", "foo", "bar");
    println!("name={}", name.clone());
    std::process::ExitCode::SUCCESS
}