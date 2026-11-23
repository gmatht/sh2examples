fn main() -> std::process::ExitCode {
    eprintln!("{}", r#"syntax error: Invalid syntax: `;;` outside case context"#);
    std::process::ExitCode::from(2)
}
