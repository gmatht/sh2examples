use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() -> std::process::ExitCode {
    { let __sh_w = [{ let __sh_split_s: String = __SH_RC.load(Ordering::Relaxed).to_string(); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    println!("{}", __SH_ARGV.lock().unwrap().join(" "));
    println!("{}", __SH_ARGV.lock().unwrap().join(" "));
    std::process::ExitCode::SUCCESS
}