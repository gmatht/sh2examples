use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    if std::path::Path::new("/etc/passwd").is_file() {
        {
let mut __sh_cur: String = ("".to_string()).to_string();
let mut __sh_rc: i32 = 0i32;
let __sh_strerr = |__e: &std::io::Error| -> String { match __e.kind() { std::io::ErrorKind::NotFound => "No such file or directory".to_string(), std::io::ErrorKind::PermissionDenied => "Permission denied".to_string(), std::io::ErrorKind::IsADirectory => "Is a directory".to_string(), _ => format!("{}", __e) } };
__sh_cur = match std::fs::read_to_string("/etc/passwd") { Ok(__s) => __s, Err(__e) => { eprintln!("cat: {}: {}", "/etc/passwd", __sh_strerr(&__e)); String::new() } };
__sh_cur = { let __src = __sh_cur.clone(); let mut __end = __src.len(); let mut __n = 0usize; for (__i, __c) in __src.char_indices() { if __c == '\n' { __n += 1; if __n == 3 { __end = __i + 1; break; } } } __src[..__end].to_string() };
__sh_cur = { let __src = __sh_cur.clone(); let __fields: Vec<usize> = vec![1usize]; let __d: char = ':'; let mut __o = String::new(); if !__src.is_empty() { let __body = __src.strip_suffix('\n') .unwrap_or(&__src).to_string(); for __l in __body.split('\n') { if !__l.contains(__d) { __o.push_str(__l); } else { let __parts: Vec<&str> = __l.split(__d).collect(); let mut __first = true; for __f in &__fields { if !__first { __o.push(__d); } __first = false; __o.push_str(if *__f >= 1 && *__f <= __parts.len() { __parts[*__f - 1] } else { "" }); } } __o.push('\n'); } } __o };
print!("{}", __sh_cur);
__SH_RC.store(__sh_rc, Ordering::Relaxed);}
    } else {
        print!("not found\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    println!("exit: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}