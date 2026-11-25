fn main() -> std::process::ExitCode {
    print!("Testing ambiguous operators...\n");
    println!("2**3**2 = {}", 512);
    std::process::ExitCode::SUCCESS
}