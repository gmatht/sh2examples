fn main() -> std::process::ExitCode {
    print!("hello\n");
    eprintln!("{}", "syntax error near unexpected token `}'");
    std::process::exit(2);
}