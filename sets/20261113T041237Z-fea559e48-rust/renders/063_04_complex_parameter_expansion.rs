use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_array: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", { let __v = std::env::var("var").unwrap_or_default(); if __v.is_empty() { format!("{}", { let __v = std::env::var("default").unwrap_or_default(); if __v.is_empty() { format!("{}", { let __v = std::env::var("fallback").unwrap_or_default(); if __v.is_empty() { "computed".to_string() } else { __v.to_string() } }) } else { __v.to_string() } }) } else { __v.to_string() } });
    println!("{}", __SHV_array.with(|v| v.borrow().get(0usize).cloned().unwrap_or_default()));
    __SH_ARITH_WORD_FAIL.store(false, Ordering::SeqCst);
    { let __ws = vec![{ __SH_ARITH_WORD_FAIL.store(true, Ordering::SeqCst); String::new() }]; let __wf = __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst); if !__wf { { let __w: &[Vec<String>] = {&[__ws]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } print!("{}", __s); } }; } __SH_RC.store(if __wf { 1 } else { 0 }, Ordering::SeqCst); }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}