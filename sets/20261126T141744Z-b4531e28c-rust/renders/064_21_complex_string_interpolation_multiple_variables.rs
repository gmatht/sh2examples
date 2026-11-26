fn __sh_hostname() -> String {
    if let Ok(h) = std::env::var("HOSTNAME") { return h; }
    std::fs::read_to_string("/proc/sys/kernel/hostname")
        .map(|s| s.trim_end().to_string()).unwrap_or_default()
}

fn main() -> std::process::ExitCode {
    let message = format!("Hello {} from {}", { let __sh_value = std::env::var("USER").unwrap_or_default(); if __sh_value.is_empty() { "guest".to_string() } else { __sh_value.to_string() } }, { let __sh_value = __sh_hostname(); if __sh_value.is_empty() { "localhost".to_string() } else { __sh_value.to_string() } });
    println!("{}", message.clone());
    std::process::ExitCode::SUCCESS
}