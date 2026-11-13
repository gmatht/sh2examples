fn main() {
    print!("Testing complex string interpolation...\n");
    let message = format!("Hello, {}! Your home is {}", { let __v = std::env::var("USER").unwrap_or_default(); if __v.is_empty() { format!("{}", { let __sc = std::process::Command::new("whoami").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }.0) } else { __v.to_string() } }, { let __v = std::env::var("HOME").unwrap_or_default(); if __v.is_empty() { format!("{}", ("~".to_string(), 0).0) } else { __v.to_string() } });
    println!("{}", message);
}