use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    upper("hello".to_string());
    upper("world".to_string());
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn upper(p0: String) {
    let mut val = String::new();
    val = { let __sh_from_set: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect(); let __sh_to_set: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); p0.clone().chars().map(|__sh_ch| match __sh_from_set.iter().position(|&__sh_from_ch| __sh_from_ch == __sh_ch) { Some(__sh_begin) => __sh_to_set[__sh_begin.min(__sh_to_set.len() - 1)], None => __sh_ch }).collect::<String>() };
    println!("{}", val.clone());
}