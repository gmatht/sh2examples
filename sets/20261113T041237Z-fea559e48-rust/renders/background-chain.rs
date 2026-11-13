use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_BG: std::sync::Mutex<Vec<(u32, std::process::Child)>> = std::sync::Mutex::new(Vec::new());
static __SH_BGTHREADS: std::sync::Mutex<Vec<std::thread::JoinHandle<()>>> = std::sync::Mutex::new(Vec::new());

fn __sh_wait_all() {
    let hs = std::mem::take(&mut *__SH_BG.lock().unwrap());
    for (_, mut h) in hs { let _ = h.wait(); }
    let ths = std::mem::take(&mut *__SH_BGTHREADS.lock().unwrap());
    for th in ths { let _ = th.join(); }
}

fn main() {
    let __th = std::thread::spawn(move || {
    print!("{}", "start\n");
});
__SH_BGTHREADS.lock().unwrap().push(__th);
    __sh_wait_all();
    println!("{}", "done".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}