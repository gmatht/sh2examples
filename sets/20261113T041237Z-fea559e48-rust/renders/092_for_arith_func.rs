use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_n: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_result: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

fn main() {
    println!("{}", factorial("5".to_string().to_string()));
    println!("{}", factorial("6".to_string().to_string()));
}

fn factorial(p0: String) -> String {
    __SHV_n.with(|v| *v.borrow_mut() = format!("{}", p0.clone()));
    __SHV_result.with(|v| v.set(1));
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_arith_v6 = { let __v = 2; __SHV_i.with(|v| v.set(__v)); __v };
    let __n7 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v6 };
    __SH_RC.store(if __n7 != 0 { 0 } else { 1 }, Ordering::SeqCst);
    let mut __sh_while_ran8 = false;
    let mut __sh_while_last9 = 0;
    while __SHV_i.with(|v| v.get()) <= __SHV_n.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) {
        __sh_while_ran8 = true;
        __SHV_result.with(|v| v.set(__SHV_result.with(|v| v.get()) * __SHV_i.with(|v| v.get())));
        __SH_RC.store(0, Ordering::SeqCst);
        let __sh_arith_v10 = { let __o = __SHV_i.with(|v| v.get()); __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) +1)); __o };
        let __n11 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v10 };
        __SH_RC.store(if __n11 != 0 { 0 } else { 1 }, Ordering::SeqCst);
        __sh_while_last9 = __SH_RC.load(Ordering::SeqCst);
    }
    if !__sh_while_ran8 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last9, Ordering::SeqCst); }
    let __v = __SHV_result.with(|v| v.get()).to_string(); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
}