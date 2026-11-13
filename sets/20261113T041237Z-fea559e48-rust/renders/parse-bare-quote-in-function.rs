use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_RESULT: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    print!("{}", format!("{}=[{}]\n", "RESULT".to_string(), __SHV_RESULT.with(|v| v.borrow().clone())));
}

fn test() {
    { let __t: &str = {"hello world\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_RESULT.with(|v| *v.borrow_mut() = {
    __SH_RC.store(0, Ordering::SeqCst);
    "test".to_string()
});
}