thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_j: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_i.with(|v| v.set(0));
    __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 1));
    __SHV_j.with(|v| v.set(__SHV_j.with(|v| v.get()) + 1));
    println!("{}", [{ let __si = __SHV_i.with(|v| v.get()).to_string(); let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }, { let __si = __SHV_j.with(|v| v.get()).to_string(); let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }].concat().join(" "));
}