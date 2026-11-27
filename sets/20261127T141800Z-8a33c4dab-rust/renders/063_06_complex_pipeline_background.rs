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

fn main() -> std::process::ExitCode {
    let __sh_bgthread = std::thread::spawn(move || {
    std::thread::sleep(std::time::Duration::from_secs_f64(("1".to_string()).trim().parse::<f64>().unwrap_or(0.0).max(0.0)));
    print!("Starting\n");
});
__SH_BGTHREADS.lock().unwrap().push(__sh_bgthread);
    let __sh_bgthread = std::thread::spawn(move || {
    std::thread::sleep(std::time::Duration::from_secs_f64(("2".to_string()).trim().parse::<f64>().unwrap_or(0.0).max(0.0)));
    print!("Processing\n");
});
__SH_BGTHREADS.lock().unwrap().push(__sh_bgthread);
    __sh_wait_all();
    print!("All done\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("exit: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}