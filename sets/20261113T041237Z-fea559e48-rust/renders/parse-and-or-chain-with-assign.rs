use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_result: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    if !({ let __r = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("-q").arg("pattern").arg("file.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }; __SH_RC.store(__r, Ordering::SeqCst); __r == 0 } && { let _ = { let __v = format!("{} match", __SHV_result.with(|v| v.borrow().clone())); __SHV_result.with(|v| *v.borrow_mut() = __v.clone()); __v }; __SH_RC.load(Ordering::SeqCst) == 0 }) {
        let _ = { let __v = format!("{} no-match", __SHV_result.with(|v| v.borrow().clone())); __SHV_result.with(|v| *v.borrow_mut() = __v.clone()); __v };
        __SH_RC.store(0, Ordering::SeqCst);
    }
    println!("{}", __SHV_result.with(|v| v.borrow().clone()));
    __SH_RC.store(0, Ordering::SeqCst);
}