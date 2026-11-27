fn main() -> std::process::ExitCode {
    println!("cmdsub={}", "sub-result");
    print!("group-run\n");
    std::process::ExitCode::SUCCESS
}