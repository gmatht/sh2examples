use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_i.with(|v| v.set(0));
    __SH_RC.store(0, Ordering::SeqCst);
    let mut __sh_while_ran0 = false;
    let mut __sh_while_last1 = 0;
    while __SHV_i.with(|v| v.get()) < 5 {
        __sh_while_ran0 = true;
        println!("{}", f(({ let __si = __SHV_i.with(|v| v.get()).to_string(); let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }.join(" ")).trim().parse::<i64>().unwrap_or(0)));
        __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 1));
        __SH_RC.store(0, Ordering::SeqCst);
        __sh_while_last1 = __SH_RC.load(Ordering::SeqCst);
    }
    if !__sh_while_ran0 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last1, Ordering::SeqCst); }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}

fn f(p0: i64) -> String {
    let __v = (p0 + p0).to_string(); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
}