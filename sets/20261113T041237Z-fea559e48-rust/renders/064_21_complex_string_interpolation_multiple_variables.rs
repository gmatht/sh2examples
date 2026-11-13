fn __sh_capture(cmd: &str) -> String {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdin(std::process::Stdio::inherit());
    match c.output() {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Err(_) => String::new(),
    }
}

fn main() {
    let message = format!("Hello {} from {}", { let __v = std::env::var("USER").unwrap_or_default(); if __v.is_empty() { "guest".to_string() } else { __v.to_string() } }, { let __v = __sh_capture("hostname"); if __v.is_empty() { "localhost".to_string() } else { __v.to_string() } });
    println!("{}", message);
}