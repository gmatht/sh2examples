thread_local! { #[allow(non_upper_case_globals)] static __SHV_array: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    c.stdin(std::process::Stdio::inherit());
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let mut s = String::from_utf8_lossy(&out).to_string();
    while s.ends_with('\n') { s.pop(); }
    (s, rc)
}

fn __sh_regex(text: &str, pat: &str, flags: &str) -> bool {
    let cmd = if flags.contains('i') {
        "shopt -s nocasematch; [[ $1 =~ $2 ]]"
    } else {
        "[[ $1 =~ $2 ]]"
    };
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd).arg("sh").arg(text).arg(pat);
    c.status().map(|s| s.success()).unwrap_or(false)
}

#[allow(non_upper_case_globals)]
fn main() {
    if (__sh_regex(&std::env::var("var").unwrap_or_default(), "^[0-9]+$", "") && std::env::var("var").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) > 0) && std::path::Path::new(&std::env::var("file").unwrap_or_default()).is_file() {
        if __sh_regex(&std::env::var("array[@]").unwrap_or_default(), "value", "") || __SHV_array.with(|v| v.borrow().len()) > 5 {
            if !__sh_capture_rc(&"echo \"$var\" | grep -q \"pattern\"").0.is_empty() {
                print!("Deeply nested condition met\n");
            } else {
            }
        } else {
        }
    } else {
    }
}