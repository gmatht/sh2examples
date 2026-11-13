use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_count: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_count.with(|v| v.set(3));
    __SH_RC.store(0, Ordering::SeqCst);
    let mut __sh_while_ran0 = false;
    let mut __sh_while_last1 = 0;
    while !(__SHV_count.with(|v| v.get()) == 0) {
        __sh_while_ran0 = true;
        println!("{}", format!("count={}", __SHV_count.with(|v| v.get())));
        __SHV_count.with(|v| v.set(__SHV_count.with(|v| v.get()) - 1));
        __SH_RC.store(0, Ordering::SeqCst);
        __sh_while_last1 = __SH_RC.load(Ordering::SeqCst);
    }
    if !__sh_while_ran0 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last1, Ordering::SeqCst); }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}