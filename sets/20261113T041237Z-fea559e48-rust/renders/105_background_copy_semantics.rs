use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_BG: std::sync::Mutex<Vec<(u32, std::process::Child)>> = std::sync::Mutex::new(Vec::new());
static __SH_BGTHREADS: std::sync::Mutex<Vec<std::thread::JoinHandle<()>>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_x: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_wait_all() {
    let hs = std::mem::take(&mut *__SH_BG.lock().unwrap());
    for (_, mut h) in hs { let _ = h.wait(); }
    let ths = std::mem::take(&mut *__SH_BGTHREADS.lock().unwrap());
    for th in ths { let _ = th.join(); }
}

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_x.with(|v| *v.borrow_mut() = "1".to_string());
    let __th = std::thread::spawn(move || {
    __SHV_x.with(|v| *v.borrow_mut() = "2".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
});
__SH_BGTHREADS.lock().unwrap().push(__th);
    __sh_wait_all();
    println!("{}", format!("x={}", __SHV_x.with(|v| v.borrow().clone())));
}