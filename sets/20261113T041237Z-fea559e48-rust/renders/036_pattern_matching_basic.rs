use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

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
    println!("{}", "== [[ pattern and regex ]]".to_string());
    let s = "file.txt";
    __SH_RC.store(0, Ordering::SeqCst);
    if !(if "file.txt".to_string().ends_with(".txt") { "1" } else { "0" }).to_string().is_empty() {
        print!("{}", "pattern-match\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    if { let __b = __sh_regex(&s, "^file\\.[a-z]+$", ""); __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b } {
        print!("{}", "regex-match\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}