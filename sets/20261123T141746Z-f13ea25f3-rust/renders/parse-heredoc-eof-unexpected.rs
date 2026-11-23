use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_VAR: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_VAR2: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_VAR.with(|v| *v.borrow_mut() = "hello".to_string());
    __SHV_VAR2.with(|v| *v.borrow_mut() = "world".to_string());
    { let __sh_c = format!("{} more {} text\n", __SHV_VAR.with(|v| v.borrow().clone()), __SHV_VAR2.with(|v| v.borrow().clone())); __sh_sink(__sh_c.as_bytes()); __SH_RC.store(0, Ordering::Relaxed); }
    print!("VAR=[hello]\n");
    print!("VAR2=[world]\n");
    std::process::ExitCode::SUCCESS
}