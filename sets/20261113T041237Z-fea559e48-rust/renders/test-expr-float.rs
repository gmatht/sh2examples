fn main() {
    if { let __t: &str = {&{ let __sc = std::process::Command::new("uname").arg("-r").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }.0}; __t.starts_with("5.4.") } {
        print!("match\n");
    } else {
    }
}