use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_BG: std::sync::Mutex<Vec<(u32, std::process::Child)>> = std::sync::Mutex::new(Vec::new());

static __SH_BGPID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn __sh_bg(cmd: &str) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    match c.spawn() {
        Ok(ch) => {
            let pid = ch.id();
            __SH_BGPID.store(pid as i32, Ordering::SeqCst);
            __SH_BG.lock().unwrap().push((pid, ch));
        }
        Err(_) => {}
    }
}

fn main() {
    __sh_bg(&format!("({}; {})", "'sleep' '1'".to_string(), "'echo' 'a'".to_string()));
    println!("{}", "b".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("exit: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}