fn main() -> std::process::ExitCode {
    eprintln!("{}", r#"syntax error: Unexpected end of input"#);
    std::process::ExitCode::from(2)
}
