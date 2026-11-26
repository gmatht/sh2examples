use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    let __sh_arith_value0 = { let __sh_value = __SHV_i.with(|v| v.get()) + 1; __SHV_i.with(|v| v.set(__sh_value)); __sh_value };
    let __sh_num1 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value0 };
    __SH_RC.store(if __sh_num1 != 0 { 0 } else { 1 }, Ordering::Relaxed);
    { let __sh_w = [{ let __sh_split_s: String = __SHV_i.with(|v| v.get()).to_string(); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    std::process::ExitCode::SUCCESS
}