use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_j: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    for __sh_lv1 in [1, 2, 3, 4, 5] {
        __SHV_i.with(|v| v.set(__sh_lv1));
        println!("Number: {}", __sh_lv1);
    }
    for __sh_w2 in [1, 2, 3] {
        __SHV_i.with(|v| v.set(__sh_w2));
        __SHV_j.with(|v| v.set(__SHV_j.with(|v| v.get()) + 1));
    }
    { let __sh_w = [{ let __sh_split_s: String = __SHV_j.with(|v| v.get()).to_string(); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    __SHV_i.with(|v| v.set(0));
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_while_ran3 = false;
    let mut __sh_while_last4 = 0;
    while __SHV_i.with(|v| v.get()) < 10 {
        __sh_while_ran3 = true;
        println!("Counter: {}", __SHV_i.with(|v| v.get()));
        __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 1));
        __SH_RC.store(0, Ordering::Relaxed);
        __sh_while_last4 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran3 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last4, Ordering::Relaxed); }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}