use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() -> std::process::ExitCode {
    let mut i = { let __sh_value = (2).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } };
    let __sh_arith_value0 = ({ let __sh_out = i.trim().parse::<i64>().unwrap_or(0); i = (i.trim().parse::<i64>().unwrap_or(0) +1).to_string(); __sh_out } + { i = (i.trim().parse::<i64>().unwrap_or(0) +1).to_string(); i.trim().parse::<i64>().unwrap_or(0) }).to_string();
    let j = __sh_arith_value0.clone();
    { let __sh_value = __sh_arith_value0; let __sh_num = __sh_value.trim().parse::<i64>().unwrap_or(0); __SH_RC.store(if __sh_num != 0 { 0 } else { 1 }, Ordering::Relaxed); __sh_value };
    println!("i={}, j={}", i.clone(), j.clone());
    std::process::ExitCode::SUCCESS
}