fn main() -> std::process::ExitCode {
    let c = 13;
    print!("a=[{}]\n", 9);
    print!("b=[{}]\n", 4);
    print!("c=[{}]\n", c);
    println!("{}", (13 + c));
    std::process::ExitCode::SUCCESS
}