use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_x: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() {
    let __sh_arith_v0 = { let __v = 5; __SHV_x.with(|v| v.set(__v)); __v };
    let __n1 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v0 };
    __SH_RC.store(if __n1 != 0 { 0 } else { 1 }, Ordering::SeqCst);
    let __sh_arith_v2 = { __SHV_x.with(|v| v.set(__SHV_x.with(|v| v.get()) + 3)); __SHV_x.with(|v| v.get()) };
    let __n3 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v2 };
    __SH_RC.store(if __n3 != 0 { 0 } else { 1 }, Ordering::SeqCst);
    println!("{}", { let __si = __SHV_x.with(|v| v.get()).to_string(); let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }.join(" "));
}