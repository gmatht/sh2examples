use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_array: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_matrix: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    let x = String::new();
    let y = String::new();
    let z = String::new();
    __SHV_matrix.with(|v| { v.borrow_mut().insert("0,0".to_string(), { let __sh_value = (((x.trim().parse::<i64>().unwrap_or(0) + y.trim().parse::<i64>().unwrap_or(0)) * z.trim().parse::<i64>().unwrap_or(0))).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } }); });
    __SHV_matrix.with(|v| { v.borrow_mut().insert("1,1".to_string(), __SHV_array.with(|v| v.borrow().get(0 as usize).cloned().unwrap_or_default())); });
    __SHV_matrix.with(|v| { v.borrow_mut().insert("2,2".to_string(), String::new()); });
    __SHV_matrix.with(|v| { v.borrow_mut().insert("3,3".to_string(), __SHV_array.with(|v| v.borrow().len().to_string())); });
    print!("Matrix assignments completed\n");
    std::process::ExitCode::SUCCESS
}