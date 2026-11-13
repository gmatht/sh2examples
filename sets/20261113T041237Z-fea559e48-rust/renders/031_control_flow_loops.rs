use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_j: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() {
    for __sh_lv1 in [1, 2, 3, 4, 5] {
        __SHV_i.with(|v| v.set(__sh_lv1));
        println!("{}", format!("Number: {}", __sh_lv1));
    }
    for __sh_w2 in [1, 2, 3] {
        __SHV_i.with(|v| v.set(__sh_w2));
        __SHV_j.with(|v| v.set(__SHV_j.with(|v| v.get()) + 1));
    }
    println!("{}", { let __si = __SHV_j.with(|v| v.get()).to_string(); let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }.join(" "));
    __SHV_i.with(|v| v.set(0));
    __SH_RC.store(0, Ordering::SeqCst);
    let mut __sh_while_ran3 = false;
    let mut __sh_while_last4 = 0;
    while __SHV_i.with(|v| v.get()) < 10 {
        __sh_while_ran3 = true;
        println!("{}", format!("Counter: {}", __SHV_i.with(|v| v.get())));
        __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 1));
        __SH_RC.store(0, Ordering::SeqCst);
        __sh_while_last4 = __SH_RC.load(Ordering::SeqCst);
    }
    if !__sh_while_ran3 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last4, Ordering::SeqCst); }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}