fn main() -> std::process::ExitCode {
    print!("1 2 3 4 5\n");
    print!("file.txt file.bak\n");
    for _i in [1, 2, 3] {
        println!("{}", _i);
    }
    std::process::ExitCode::SUCCESS
}