use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_array: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_matrix: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_x: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_y: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_z: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_matrix.with(|v| { v.borrow_mut().insert("0,0".to_string(), { let __v = (((__SHV_x.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + __SHV_y.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0)) * __SHV_z.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0))).to_string(); let __e = __SH_ARITH_ERR.swap(false, Ordering::SeqCst); __SH_RC.store(if __e { 1 } else { 0 }, Ordering::SeqCst); if __e { String::new() } else { __v } }); });
    __SHV_matrix.with(|v| { v.borrow_mut().insert("1,1".to_string(), __SHV_array.with(|v| v.borrow().get(0usize).cloned().unwrap_or_default())); });
    __SHV_matrix.with(|v| { v.borrow_mut().insert("2,2".to_string(), ["prefix"].join(" ")); });
    __SHV_matrix.with(|v| { v.borrow_mut().insert("3,3".to_string(), (__SHV_array.with(|v| v.borrow().len())).to_string()); });
    println!("{}", "Matrix assignments completed".to_string());
}