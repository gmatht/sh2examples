use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV___n: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() {
    #[allow(unused_assignments)] let mut i = 1;
    __SHV___n.with(|v| v.set(0));
    while __SHV___n.with(|v| v.get()) < 1000 {
        let __sh_arith_v2 = { let __o = i; i = i +1; __o };
        let __n3 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v2 };
        __SH_RC.store(if __n3 != 0 { 0 } else { 1 }, Ordering::SeqCst);
        __SHV___n.with(|v| v.set(__SHV___n.with(|v| v.get()) + 1));
    }
    println!("{}", format!("increment={}", i));
}