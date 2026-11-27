fn main() -> std::process::ExitCode {
    eprintln!("{}", r#"syntax error: Invalid syntax: unterminated `${` in double-quoted string"#);
    std::process::ExitCode::from(2)
}
