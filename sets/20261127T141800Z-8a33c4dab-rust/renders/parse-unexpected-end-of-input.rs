fn main() -> std::process::ExitCode {
    eprintln!("{}", r#"syntax error: Lexer error: Unexpected character: ? at 6:17"#);
    std::process::ExitCode::from(2)
}
