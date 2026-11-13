use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() {
    #[allow(unused_assignments)] let mut i = { let __v = (2).to_string(); let __e = __SH_ARITH_ERR.swap(false, Ordering::SeqCst); __SH_RC.store(if __e { 1 } else { 0 }, Ordering::SeqCst); if __e { String::new() } else { __v } };
    let __sh_arith_v0 = ({ let __o = i.clone().trim().parse::<i64>().unwrap_or(0); i = (i.clone().trim().parse::<i64>().unwrap_or(0) +1).to_string(); __o } + { i = (i.clone().trim().parse::<i64>().unwrap_or(0) +1).to_string(); i.clone().trim().parse::<i64>().unwrap_or(0) }).to_string();
    let j = __sh_arith_v0.clone();
    let _ = { let __v = __sh_arith_v0; let __n = __v.trim().parse::<i64>().unwrap_or(0); __SH_RC.store(if __n != 0 { 0 } else { 1 }, Ordering::SeqCst); __v };
    println!("{}", format!("i={}, j={}", i.clone(), j.clone()));
}