use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    print!("== Here-string with grep -o ==\n");
    __SH_RC.store(0, Ordering::Relaxed);
    let (__sh_mapped, __sh_res) = { let __sh_text = "some pattern here"; let __sh_num = __sh_text.matches("pattern").count(); (if __sh_num == 0 { String::new() } else { vec!["pattern"; __sh_num].join("\n") }, if __sh_num == 0 { 1 } else { 0 }) };
    if !__sh_mapped.is_empty() { println!("{}", __sh_mapped); }
    __SH_RC.store(__sh_res, Ordering::Relaxed);
    println!("exit: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}