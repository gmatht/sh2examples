use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    {
let mut __sh_cur: String = ("select\n".to_string()).to_string();
let mut __sh_rc: i32 = 0i32;
__sh_cur = { let __src = __sh_cur.clone(); let mut __end = __src.len(); let mut __n = 0usize; for (__i, __c) in __src.char_indices() { if __c == '\n' { __n += 1; if __n == 1 { __end = __i + 1; break; } } } __src[..__end].to_string() };
print!("{}", __sh_cur);
__SH_RC.store(__sh_rc, Ordering::Relaxed);}
    println!("exit: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}