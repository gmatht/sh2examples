use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_a: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_b: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_c: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_d: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    let x = { let __v = ({ let __a = (__SHV_a.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + __SHV_b.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0)); let __b = (__SHV_c.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + __SHV_d.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0)); if __b == 0 { __SH_ARITH_ERR.store(true, Ordering::SeqCst); 0 } else { __a / __b } }).to_string(); let __e = __SH_ARITH_ERR.swap(false, Ordering::SeqCst); __SH_RC.store(if __e { 1 } else { 0 }, Ordering::SeqCst); if __e { String::new() } else { __v } };
    print!("{}", format!("{}=[{}]\n", "x".to_string(), x.clone()));
}