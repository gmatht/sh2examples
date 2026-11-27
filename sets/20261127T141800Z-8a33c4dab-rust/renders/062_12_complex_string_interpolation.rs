fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    let Ok(mut ch) = c.spawn() else { return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let s = String::from_utf8_lossy(&out).trim_end_matches('\n').to_string();
    (s, rc)
}

fn main() -> std::process::ExitCode {
    print!("Testing complex string interpolation...\n");
    let message = format!("Hello, {}! Your home is {}", { let __sh_value = std::env::var("USER").unwrap_or_default(); if __sh_value.is_empty() { format!("{}", __sh_capture_rc(&"whoami").0) } else { __sh_value.to_string() } }, { let __sh_value = std::env::var("HOME").unwrap_or_default(); if __sh_value.is_empty() { format!("{}", __sh_capture_rc(&"echo ~").0) } else { __sh_value.to_string() } });
    println!("{}", message.clone());
    std::process::ExitCode::SUCCESS
}