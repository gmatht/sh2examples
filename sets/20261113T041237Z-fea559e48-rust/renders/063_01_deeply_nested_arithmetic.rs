use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_a: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_b: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_c: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_d: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_e: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_f: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_g: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_h: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_j: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_k: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_l: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_m: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_n: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

fn __sh_pow(a: i64, b: i64) -> i64 {
    let mut r: i64 = 1;
    let mut i: i64 = 0;
    while i < b { r = r.wrapping_mul(a); i += 1; }
    r
}

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_a.with(|v| v.set(10));
    __SHV_b.with(|v| v.set(3));
    __SHV_c.with(|v| v.set(8));
    __SHV_d.with(|v| v.set(2));
    __SHV_e.with(|v| v.set(7));
    __SHV_f.with(|v| v.set(2));
    __SHV_g.with(|v| v.set(2));
    __SHV_h.with(|v| v.set(3));
    __SHV_i.with(|v| v.set(4));
    __SHV_j.with(|v| v.set(1));
    __SHV_k.with(|v| v.set(6));
    __SHV_l.with(|v| v.set(3));
    __SHV_m.with(|v| v.set(5));
    __SHV_n.with(|v| v.set(2));
    let result = { let __v = (((({ let __a = ((__SHV_a.with(|v| v.get()) + __SHV_b.with(|v| v.get())) * (__SHV_c.with(|v| v.get()) - __SHV_d.with(|v| v.get()))); let __b = { let __a = __SHV_e.with(|v| v.get()); let __b = __SHV_f.with(|v| v.get()); if __b == 0 { __SH_ARITH_ERR.store(true, Ordering::SeqCst); 0 } else { __a % __b } }; if __b == 0 { __SH_ARITH_ERR.store(true, Ordering::SeqCst); 0 } else { __a / __b } } + __sh_pow(__SHV_g.with(|v| v.get()),__SHV_h.with(|v| v.get()))) - (__SHV_i.with(|v| v.get()) << __SHV_j.with(|v| v.get()))) | ((__SHV_k.with(|v| v.get()) & __SHV_l.with(|v| v.get())) ^ (__SHV_m.with(|v| v.get()) | __SHV_n.with(|v| v.get()))))).to_string(); let __e = __SH_ARITH_ERR.swap(false, Ordering::SeqCst); __SH_RC.store(if __e { 1 } else { 0 }, Ordering::SeqCst); if __e { String::new() } else { __v } };
    println!("{}", format!("Deeply nested arithmetic result: {}", result.clone()));
}