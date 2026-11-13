use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    let _ = { let __b = "abc" == "abc"; __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b };
    println!("{}", format!("eq={}", __SH_RC.load(Ordering::SeqCst).to_string()));
    let _ = { let __b = "abc" != "xyz"; __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b };
    println!("{}", format!("ne={}", __SH_RC.load(Ordering::SeqCst).to_string()));
    let _ = { let __b = 5 < 10; __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b };
    println!("{}", format!("lt={}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}