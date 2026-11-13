thread_local! { #[allow(non_upper_case_globals)] static __SHV_tmp: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    let __sh_case0 = std::env::var("opt").unwrap_or_default();
    if (__sh_case0 == "") {
    } else if (__sh_case0.starts_with("configfile=")) {
        __SHV_tmp.with(|v| *v.borrow_mut() = { let __v = {&std::env::var("opt").unwrap_or_default()}.to_string(); match __v.find("=") { Some(__i) => __v[__i + "=".len()..].to_string(), None => __v } });
    } else if true {
        print!("default\n");
    } else {
    }
    print!("{}", format!("{}=[{}]\n", "tmp".to_string(), __SHV_tmp.with(|v| v.borrow().clone())));
    print!("{}", format!("{}=[{}]\n", "configfile".to_string(), std::env::var("configfile").unwrap_or_default()));
}