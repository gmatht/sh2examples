use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
}

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_i.with(|v| v.set(0));
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_while_ran0 = false;
    let mut __sh_while_last1 = 0;
    while __SHV_i.with(|v| v.get()) < 5 {
        __sh_while_ran0 = true;
        println!("{}", f((({ let __sh_split_s: String = __SHV_i.with(|v| v.get()).to_string(); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }).join(" ")).trim().parse::<i64>().unwrap_or(0)));
        __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 1));
        __SH_RC.store(0, Ordering::Relaxed);
        __sh_while_last1 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran0 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last1, Ordering::Relaxed); }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn f(p0: i64) -> String {
    let __sh_value = (p0 + p0).to_string(); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed); return __sh_value;
}