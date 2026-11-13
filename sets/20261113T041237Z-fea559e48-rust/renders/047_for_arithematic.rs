thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_j: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_j.with(|v| *v.borrow_mut() = "1".to_string());
    for __sh_w0 in ["1", "2", "3", "4", "5"] {
        __SHV_i.with(|v| *v.borrow_mut() = __sh_w0.to_string());
        __SHV_j.with(|v| *v.borrow_mut() = (__SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) * __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0)).to_string());
    }
    println!("{}", { let __si = __SHV_j.with(|v| v.borrow().clone()); let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }.join(" "));
}