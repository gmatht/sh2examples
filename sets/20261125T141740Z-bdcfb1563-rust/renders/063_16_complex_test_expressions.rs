fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    c.stdin(std::process::Stdio::inherit());
    let Ok(mut ch) = c.spawn() else { return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let mut s = String::from_utf8_lossy(&out).to_string();
    while s.ends_with('\n') { s.pop(); }
    (s, rc)
}

fn main() -> std::process::ExitCode {
    if ((!std::env::var("var").unwrap_or_default().is_empty()) && std::path::Path::new(&std::env::var("file").unwrap_or_default()).is_file()) || std::path::Path::new(&std::env::var("dir").unwrap_or_default()).is_dir() && __sh_capture_rc(&"wc -l < \"$file\"").0.trim().parse::<i64>().unwrap_or(0) > 10 {
        print!("Complex test passed\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}