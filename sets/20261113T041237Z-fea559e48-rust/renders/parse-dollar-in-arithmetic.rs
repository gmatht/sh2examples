use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() {
    __SH_ARITH_WORD_FAIL.store(false, Ordering::SeqCst);
    { let __ws = vec![{ let __aw = ((({ let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() }.trim().parse::<i64>().unwrap_or(0) * 100) + { let __g = __SH_ARGV.lock().unwrap(); __g.get(1).cloned().unwrap_or_default() }.trim().parse::<i64>().unwrap_or(0))).to_string(); if { let __g = __SH_ARGV.lock().unwrap(); __g.get(0).cloned().unwrap_or_default() }.trim().is_empty() || { let __g = __SH_ARGV.lock().unwrap(); __g.get(1).cloned().unwrap_or_default() }.trim().is_empty() { __SH_ARITH_WORD_FAIL.store(true, Ordering::SeqCst); } __aw }]; let __wf = __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst); if !__wf { { let __w: &[Vec<String>] = {&[__ws]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } print!("{}", __s); } }; } __SH_RC.store(if __wf { 1 } else { 0 }, Ordering::SeqCst); }
    { let __w: &[Vec<String>] = {&[vec![format!("{}=[{}]\n", "n".to_string(), std::env::var("n").unwrap_or_default())]]}; let __nl = {false}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } print!("{}", __s); } };
}