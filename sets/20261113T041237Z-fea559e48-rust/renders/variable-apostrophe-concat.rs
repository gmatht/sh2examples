thread_local! { #[allow(non_upper_case_globals)] static __SHV_y: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_y.with(|v| *v.borrow_mut() = format!("{}world", "hello".to_string()));
    println!("{}", { let __si = __SHV_y.with(|v| v.borrow().clone()); let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }.join(" "));
    print!("{}", format!("{}=[{}]\n", "x".to_string(), "hello".to_string()));
}