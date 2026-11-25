thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_j: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_i.with(|v| v.set(0));
    __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 1));
    __SHV_j.with(|v| v.set(__SHV_j.with(|v| v.get()) + 1));
    { let __sh_w = [{ let __sh_split_s: String = __SHV_i.with(|v| v.get()).to_string(); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }, { let __sh_split_s: String = __SHV_j.with(|v| v.get()).to_string(); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    std::process::ExitCode::SUCCESS
}