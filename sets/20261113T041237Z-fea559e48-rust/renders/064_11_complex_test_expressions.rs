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

fn main() {
    let n = 42;
    if __sh_regex(&n.to_string(), "^[0-9]+$", "") && ((!("debug".to_string() != "test".to_string())) || (!("debug".to_string() != "debug".to_string()))) {
        print!("Valid input\n");
    } else {
    }
}