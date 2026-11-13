use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() {
    #[allow(unused_assignments)] let mut i = 0;
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_arith_v0 = { let __o = i; i = i +1; __o };
    let __n1 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v0 };
    __SH_RC.store(if __n1 != 0 { 0 } else { 1 }, Ordering::SeqCst);
    println!("{}", format!("post0={}", __SH_RC.load(Ordering::SeqCst).to_string()));
    let __sh_arith_v2 = { i = i +1; i };
    let __n3 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v2 };
    __SH_RC.store(if __n3 != 0 { 0 } else { 1 }, Ordering::SeqCst);
    println!("{}", format!("pre={}", __SH_RC.load(Ordering::SeqCst).to_string()));
    i = 5;
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_arith_v4 = { let __o = i; i = i +1; __o };
    let __n5 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v4 };
    __SH_RC.store(if __n5 != 0 { 0 } else { 1 }, Ordering::SeqCst);
    println!("{}", format!("post5={}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}