use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    test_locals("hello".to_string(), "world".to_string());
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn test_locals(p0: String, p1: String) {
    let var1 = p0.clone();
    let var2 = { let __sh_value = p1.clone(); if __sh_value.is_empty() { "default_value".to_string() } else { __sh_value.to_string() } };
    let var3 = { let __sh_from_set: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect(); let __sh_to_set: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); var1.clone().chars().map(|__sh_ch| match __sh_from_set.iter().position(|&__sh_from_ch| __sh_from_ch == __sh_ch) { Some(__sh_begin) => __sh_to_set[__sh_begin.min(__sh_to_set.len() - 1)], None => __sh_ch }).collect::<String>() };
    println!("Var1: {}", var1.clone());
    println!("Var2: {}", var2.clone());
    println!("Var3: {}", var3.clone());
}