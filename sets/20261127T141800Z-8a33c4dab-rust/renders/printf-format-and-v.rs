fn main() -> std::process::ExitCode {
    let name: String;
    print!("hi 42\n");
    print!("a\tb\n");
    name = format!("{}-{}", "foo", "bar");
    println!("name={}", name.clone());
    std::process::ExitCode::SUCCESS
}