fn main() -> std::process::ExitCode {
    let x = format!("{}/file", std::env::var("HOME").unwrap_or_default());
    let z = format!("{} with {} and {}ext", x.clone(), "literal string", x.clone());
    println!("{}", z.clone());
    print!("y=[literal string]\n");
    print!("x=[{}]\n", x.clone());
    std::process::ExitCode::SUCCESS
}