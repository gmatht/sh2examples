use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    println!("{}", { let __sh_text: String = "c\na\nb".to_string(); let mut __sh_lines: Vec<String> = if __sh_text.is_empty() { Vec::new() } else { let mut __sh_collected: Vec<String> = __sh_text.split('\n').map(|__sh_line| __sh_line.to_string()).collect(); if __sh_text.ends_with('\n') { __sh_collected.pop(); } __sh_collected }; let __sort_key = |__sh_line: &String| -> String { __sh_line.clone() }; __sh_lines.sort_by(|__sh_left, __sh_right| __sort_key(__sh_left).cmp(&__sort_key(__sh_right)));  __sh_lines.join("\n") });
    __SH_RC.store((0) as i32, Ordering::Relaxed);
    println!("exit: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}